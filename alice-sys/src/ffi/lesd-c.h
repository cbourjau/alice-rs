typedef void CEsd;
typedef __SIZE_TYPE__ size_t;
typedef __SIZE_TYPE__ uint32_t;
typedef struct primary_vertex_t primary_vertex_t;
typedef struct track_t track_t;
typedef struct ext_track_parameters_t ext_track_parameters_t;

struct primary_vertex_t {
  double x;
  double y;
  double z;
  int n_contrib;
};

struct ext_track_parameters_t {
  double loc_y;
  double loc_z;
  double loc_sin;
  double tang;
  double one_over_pt;
};

struct track_t {
  ext_track_parameters_t ext_track_paras;
  double alpha;
  double x;
  double kinkIndices[3];
  uint32_t flags;
};

#ifdef __cplusplus
extern "C" {
#endif
  CEsd * esd_new(const char*);
  int esd_load_next(const CEsd* cesd, const long ievent);
  // Primary Vertex
  primary_vertex_t primary_vertex_get_pos(const CEsd* cesd);
  // Load the parameters from the Tracks_ branch
  void get_ext_tracks_parameters(const CEsd* cesd, track_t *tracks, size_t ntracks);
  // Get N tracks
  size_t get_n_tracks(const CEsd* cesd);
  // Destructor
  void esd_destroy(const CEsd* cesd);
  // Get the multiplicity of the current event
  int get_multiplicity(const CEsd* cesd);
#ifdef __cplusplus
}
#endif

