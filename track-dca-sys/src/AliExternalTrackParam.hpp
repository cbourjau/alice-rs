#ifndef ALIEXTERNALTRACKPARAM_H
#define ALIEXTERNALTRACKPARAM_H
/* Copyright(c) 1998-1999, ALICE Experiment at CERN, All rights reserved. *
 * See cxx source for full Copyright notice                               */

/* $Id$ */

/*****************************************************************************
 *              "External" track parametrisation class                       *
 *                                                                           *
 *      external param0:   local Y-coordinate of a track (cm)                *
 *      external param1:   local Z-coordinate of a track (cm)                *
 *      external param2:   local sine of the track momentum azimuthal angle  *
 *      external param3:   tangent of the track momentum dip angle           *
 *      external param4:   1/pt (1/(GeV/c))                                  *
 *                                                                           *
 * The parameters are estimated at an exact position x in a local coord.     *
 * system rotated by angle alpha with respect to the global coord.system.    *
 *        Origin: I.Belikov, CERN, Jouri.Belikov@cern.ch                     *
 *****************************************************************************/

const double kB2C = -0.299792458e-3;

class AliExternalTrackParam {
public:
  virtual double GetC(double b) const { return fP[4] * b * kB2C; }
  void GetDZ(double x, double y, double z, double b, float dz[2]) const;
  // double GetSigmaY2() const {return fC[0];}
  // double GetSigmaZ2() const {return fC[2];}
  AliExternalTrackParam(double x, double alpha, const double param[5]
                        // const double covar[15]
  );

  double GetSign() const { return (fP[4] > 0) ? 1 : -1; }
  void GetHelixParameters(double h[6], double b) const;
  double GetDCA(const AliExternalTrackParam *p, double b, double &xthis,
                double &xp) const;

protected:
  double fX;     // X coordinate for the point of parametrisation
  double fAlpha; // Local <-->global coor.system rotation angle
  double fP[5];  // The track parameters
  // double           fC[15]; // The track parameter covariance matrix
};

#endif
