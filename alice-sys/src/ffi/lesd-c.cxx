#include "stdlib.h"
#include "TTree.h"

#include "cpp_src/ESDmerged.h"
#include "lesd-c.h"

typedef void CEsd;

#ifdef __cplusplus
extern "C" {
#endif
  CEsd * esd_new(const char* path) {
    ESD* esd = new ESD(path);
    return (CEsd *)esd;
  }
  int esd_load_next(const CEsd* cesd, const long ievent) {
    ESD* esd = (ESD*)cesd;
    return esd->GetEntry(ievent);
  }
  primary_vertex_t primary_vertex_get_pos(const CEsd* cesd) {
    ESD* esd = (ESD*)cesd;
    primary_vertex_t pv =
      {
       esd->PrimaryVertex_AliVertex_fPosition[0],
       esd->PrimaryVertex_AliVertex_fPosition[1],
       esd->PrimaryVertex_AliVertex_fPosition[2],
       esd->PrimaryVertex_AliVertex_fNContributors,
      };
    return pv;
  }

  void get_ext_tracks_parameters(const CEsd* cesd, track_t *tracks, size_t ntracks) {
    ESD* esd = (ESD*)cesd;
    for (int i = 0; i < ntracks; i++) {
      tracks[i].ext_track_paras.loc_y = esd->Tracks_fP[i][0];
      tracks[i].ext_track_paras.loc_z = esd->Tracks_fP[i][1];
      tracks[i].ext_track_paras.loc_sin = esd->Tracks_fP[i][2];
      tracks[i].ext_track_paras.tang = esd->Tracks_fP[i][3];
      tracks[i].ext_track_paras.one_over_pt = esd->Tracks_fP[i][4];
      tracks[i].alpha = esd->Tracks_fAlpha[i];
      tracks[i].flags = esd->Tracks_fFlags[i];
      tracks[i].x = esd->Tracks_fX[i];
    }
  }

  size_t get_n_tracks(const CEsd* cesd) {
    ESD* esd = (ESD*)cesd;
    return esd->Tracks_;
  }

  void esd_destroy(const CEsd* cesd) {
    ESD* esd = (ESD*)cesd;
    esd->fChain->Delete();
    delete esd;
  }

  int get_multiplicity(const CEsd* cesd) {
    ESD* esd = (ESD*)cesd;
    return esd->AliMultiplicity_fNtracks;
  }
#ifdef __cplusplus
}
#endif
