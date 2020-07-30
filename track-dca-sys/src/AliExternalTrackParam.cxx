///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Implementation of the external track parameterisation class.              //
//                                                                           //
// This parameterisation is used to exchange tracks between the detectors.   //
// A set of functions returning the position and the momentum of tracks      //
// in the global coordinate system as well as the track impact parameters    //
// are implemented.
// Origin: I.Belikov, CERN, Jouri.Belikov@cern.ch                            //
///////////////////////////////////////////////////////////////////////////////
#include "AliExternalTrackParam.hpp"

#include <cmath>

const double kAlmost0 = 1.e-13;

constexpr double Pi() { return 3.14159265358979323846; }

double ASin(double x) {
  if (x < -1.)
    return -Pi() / 2;
  if (x > 1.)
    return Pi() / 2;
  return asin(x);
}

//_____________________________________________________________________________
AliExternalTrackParam::AliExternalTrackParam(double x, double alpha,
                                             const double param[5]
                                             // const double covar[15]
                                             )
    : fX(x), fAlpha(alpha) {
  //
  // create external track parameters from given arguments
  //
  for (int i = 0; i < 5; i++)
    fP[i] = param[i];
  // for (int i = 0; i < 15; i++) fC[i] = covar[i];
}

void AliExternalTrackParam::GetHelixParameters(double hlx[6], double b) const {
  //--------------------------------------------------------------------
  // External track parameters -> helix parameters
  // "b" - magnetic field (kG)
  //--------------------------------------------------------------------
  double cs = cos(fAlpha), sn = sin(fAlpha);

  hlx[0] = fP[0];
  hlx[1] = fP[1];
  hlx[2] = fP[2];
  hlx[3] = fP[3];

  hlx[5] = fX * cs - hlx[0] * sn; // x0
  hlx[0] = fX * sn + hlx[0] * cs; // y0
  // hlx[1]=                                 // z0
  hlx[2] = ASin(hlx[2]) + fAlpha; // phi0
  // hlx[3]=                                 // tgl
  hlx[4] = GetC(b); // C
}

static void Evaluate(const double *h, double t,
                     double r[3],  // radius vector
                     double g[3],  // first defivatives
                     double gg[3]) // second derivatives
{
  //--------------------------------------------------------------------
  // Calculate position of a point on a track and some derivatives
  //--------------------------------------------------------------------
  double phase = h[4] * t + h[2];
  double sn = sin(phase), cs = cos(phase);

  r[0] = h[5];
  r[1] = h[0];
  if (std::abs(h[4]) > kAlmost0) {
    r[0] += (sn - h[6]) / h[4];
    r[1] -= (cs - h[7]) / h[4];
  } else {
    r[0] += t * cs;
    r[1] -= -t * sn;
  }
  r[2] = h[1] + h[3] * t;

  g[0] = cs;
  g[1] = sn;
  g[2] = h[3];

  gg[0] = -h[4] * sn;
  gg[1] = h[4] * cs;
  gg[2] = 0.;
}

double AliExternalTrackParam::GetDCA(const AliExternalTrackParam *p, double b,
                                     double &xthis, double &xp) const {
  //------------------------------------------------------------
  // Returns the (weighed !) distance of closest approach between
  // this track and the track "p".
  // Other returned values:
  //   xthis, xt - coordinates of tracks' reference planes at the DCA
  //-----------------------------------------------------------
  double dy2 = 1.0; // GetSigmaY2() + p->GetSigmaY2();
  double dz2 = 1.0; // GetSigmaZ2() + p->GetSigmaZ2();
  double dx2 = dy2;

  double p1[8];
  GetHelixParameters(p1, b);
  p1[6] = sin(p1[2]);
  p1[7] = cos(p1[2]);
  double p2[8];
  p->GetHelixParameters(p2, b);
  p2[6] = sin(p2[2]);
  p2[7] = cos(p2[2]);

  double r1[3], g1[3], gg1[3];
  double t1 = 0.;
  Evaluate(p1, t1, r1, g1, gg1);
  double r2[3], g2[3], gg2[3];
  double t2 = 0.;
  Evaluate(p2, t2, r2, g2, gg2);

  double dx = r2[0] - r1[0], dy = r2[1] - r1[1], dz = r2[2] - r1[2];
  double dm = dx * dx / dx2 + dy * dy / dy2 + dz * dz / dz2;

  int max = 27;
  while (max--) {
    double gt1 = -(dx * g1[0] / dx2 + dy * g1[1] / dy2 + dz * g1[2] / dz2);
    double gt2 = +(dx * g2[0] / dx2 + dy * g2[1] / dy2 + dz * g2[2] / dz2);
    double h11 = (g1[0] * g1[0] - dx * gg1[0]) / dx2 +
                 (g1[1] * g1[1] - dy * gg1[1]) / dy2 +
                 (g1[2] * g1[2] - dz * gg1[2]) / dz2;
    double h22 = (g2[0] * g2[0] + dx * gg2[0]) / dx2 +
                 (g2[1] * g2[1] + dy * gg2[1]) / dy2 +
                 (g2[2] * g2[2] + dz * gg2[2]) / dz2;
    double h12 =
        -(g1[0] * g2[0] / dx2 + g1[1] * g2[1] / dy2 + g1[2] * g2[2] / dz2);

    double det = h11 * h22 - h12 * h12;

    double dt1, dt2;
    if (std::abs(det) < 1.e-33) {
      //(quasi)singular Hessian
      dt1 = -gt1;
      dt2 = -gt2;
    } else {
      dt1 = -(gt1 * h22 - gt2 * h12) / det;
      dt2 = -(h11 * gt2 - h12 * gt1) / det;
    }

    if ((dt1 * gt1 + dt2 * gt2) > 0) {
      dt1 = -dt1;
      dt2 = -dt2;
    }

    // check delta(phase1) ?
    // check delta(phase2) ?

    if (std::abs(dt1) / (std::abs(t1) + 1.e-3) < 1.e-4)
      if (std::abs(dt2) / (std::abs(t2) + 1.e-3) < 1.e-4) {
        // if ((gt1*gt1+gt2*gt2) > 1.e-4/dy2/dy2)
        // 	 AliDebug(1," stopped at not a stationary point !");
        double lmb = h11 + h22;
        lmb = lmb - sqrt(lmb * lmb - 4 * det);
        // if (lmb < 0.)
        //   AliDebug(1," stopped at not a minimum !");
        break;
      }

    double dd = dm;
    for (int div = 1;; div *= 2) {
      Evaluate(p1, t1 + dt1, r1, g1, gg1);
      Evaluate(p2, t2 + dt2, r2, g2, gg2);
      dx = r2[0] - r1[0];
      dy = r2[1] - r1[1];
      dz = r2[2] - r1[2];
      dd = dx * dx / dx2 + dy * dy / dy2 + dz * dz / dz2;
      if (dd < dm)
        break;
      dt1 *= 0.5;
      dt2 *= 0.5;
      if (div>512) {
        break;
      }
    }
    dm = dd;

    t1 += dt1;
    t2 += dt2;
  }

  // if (max<=0) AliDebug(1," too many iterations !");

  double cs = cos(fAlpha);
  double sn = sin(fAlpha);
  xthis = r1[0] * cs + r1[1] * sn;

  cs = cos(p->fAlpha);
  sn = sin(p->fAlpha);
  xp = r2[0] * cs + r2[1] * sn;

  return sqrt(dm * sqrt(dy2 * dz2));
}
