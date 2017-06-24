# Makefile for ESD -- automatically generated

# Include this file from the ROOT installed on the system where we are using the PARfile
include $(shell root-config --etcdir)/Makefile.arch

# Name of this module
PAR_MODULE := ESD

# This module depends on the following libraries
PAR_LIBDEPS := libSTEERBase libCore libEG libGeom libGpad libGraf3d libMathCore libMatrix libPhysics libRIO libTree

# Current OS
OS := $(shell uname)

# ROOT major version
ROOT_MAJOR_VERSION := $(shell root-config --version | cut -f1 -d.)

# Include paths: take into consideration "extra includes" (manually entered by users) and includes
# from other previously enabled PARfiles
PAR_EXTRAINCLUDES = 
PAR_INCLUDES = -I. $(addprefix -I../,$(ALICE_PARFILES_ENABLED)) $(addprefix -I,$(PAR_EXTRAINCLUDES)) -I$(ALICE_ROOT)/include -I$(ALICE_PHYSICS)/include -I$(shell root-config --incdir)
ifneq ($(FASTJET),)
	PAR_INCLUDES += -I$(FASTJET)/include
endif

# Get C++ flags form ROOT
PAR_CXXFLAGS = $(CXXFLAGS) $(PAR_INCLUDES)

# Get linker flags from ROOT
PAR_SOFLAGS = $(SOFLAGS) $(LDFLAGS)
ifeq ($(OS),Darwin)
# Only on OS X
	PAR_SOFLAGS += -undefined dynamic_lookup
endif

# Get some ROOT build flags from the current installation
ROOT_DEFINES += $(shell root-config --features | grep -q xml && echo '-DWITHXML')
ROOT_DEFINES += $(shell root-config --features | grep -q alien && echo '-DWITHALIEN')
ifneq ($(FASTJET),)
	ROOT_DEFINES += -DHAVE_FASTJET
endif

SRCS := AliCascadeVertexer.cxx AliESDACORDE.cxx AliESDAD.cxx AliESDADfriend.cxx AliESDCaloCells.cxx AliESDCaloCluster.cxx AliESDCaloTrigger.cxx AliESDcascade.cxx AliESDCosmicTrack.cxx AliESD.cxx AliESDEvent.cxx AliESDFIT.cxx AliESDFMD.cxx AliESDfriend.cxx AliESDfriendTrack.cxx AliESDHandler.cxx AliESDHeader.cxx AliESDHLTDecision.cxx AliESDHLTtrack.cxx AliESDInputHandler.cxx AliESDInputHandlerRP.cxx AliESDkink.cxx AliESDMultITS.cxx AliESDMuonCluster.cxx AliESDMuonGlobalTrack.cxx AliESDMuonPad.cxx AliESDMuonTrack.cxx AliESDpid.cxx AliESDPmdTrack.cxx AliESDRun.cxx AliESDTOFCluster.cxx AliESDTOFHit.cxx AliESDTOFMatch.cxx AliESDtrack.cxx AliESDTrdTrack.cxx AliESDTrdTracklet.cxx AliESDTrdTrigger.cxx AliESDTZERO.cxx AliESDTZEROfriend.cxx AliESDUtils.cxx AliESDv0.cxx AliESDV0Params.cxx AliESDVertex.cxx AliESDVZERO.cxx AliESDVZEROfriend.cxx AliESDZDC.cxx AliExpression.cxx AliFMDFloatMap.cxx AliFMDMap.cxx AliKalmanTrack.cxx AliKFParticleBase.cxx AliKFParticle.cxx AliKFVertex.cxx AliMeanVertex.cxx AliMultiplicity.cxx AliRawDataErrorLog.cxx AliSelector.cxx AliStrLine.cxx AliTrackerBase.cxx AliTrackPointArray.cxx AliTriggerBCMask.cxx AliTriggerClass.cxx AliTriggerCluster.cxx AliTriggerConfiguration.cxx AliTriggerDescriptor.cxx AliTriggerInput.cxx AliTriggerInteraction.cxx AliTriggerIR.cxx AliTriggerPFProtection.cxx AliTriggerScalersESD.cxx AliTriggerScalersRecordESD.cxx AliV0vertexer.cxx AliVertex.cxx AliVertexerTracks.cxx
HDRS := AliCascadeVertexer.h AliESDACORDE.h AliESDAD.h AliESDADfriend.h AliESDCaloCells.h AliESDCaloCluster.h AliESDCaloTrigger.h AliESDcascade.h AliESDCosmicTrack.h AliESD.h AliESDEvent.h AliESDFIT.h AliESDFMD.h AliESDfriend.h AliESDfriendTrack.h AliESDHandler.h AliESDHeader.h AliESDHLTDecision.h AliESDHLTtrack.h AliESDInputHandler.h AliESDInputHandlerRP.h AliESDkink.h AliESDMultITS.h AliESDMuonCluster.h AliESDMuonGlobalTrack.h AliESDMuonPad.h AliESDMuonTrack.h AliESDpid.h AliESDPmdTrack.h AliESDRun.h AliESDTOFCluster.h AliESDTOFHit.h AliESDTOFMatch.h AliESDtrack.h AliESDTrdTrack.h AliESDTrdTracklet.h AliESDTrdTrigger.h AliESDTZERO.h AliESDTZEROfriend.h AliESDUtils.h AliESDv0.h AliESDV0Params.h AliESDVertex.h AliESDVZERO.h AliESDVZEROfriend.h AliESDZDC.h AliExpression.h AliFMDFloatMap.h AliFMDMap.h AliKalmanTrack.h AliKFParticleBase.h AliKFParticle.h AliKFVertex.h AliMeanVertex.h AliMultiplicity.h AliRawDataErrorLog.h AliSelector.h AliStrLine.h AliTrackerBase.h AliTrackPointArray.h AliTriggerBCMask.h AliTriggerClass.h AliTriggerCluster.h AliTriggerConfiguration.h AliTriggerDescriptor.h AliTriggerInput.h AliTriggerInteraction.h AliTriggerIR.h AliTriggerPFProtection.h AliTriggerScalersESD.h AliTriggerScalersRecordESD.h AliV0vertexer.h AliVertex.h AliVertexerTracks.h

DHDR := $(PAR_MODULE)LinkDef.h

SRCS += G__$(PAR_MODULE).cxx
OBJS := $(SRCS:.cxx=.o)

COLM := \033[35m
COLC := \033[36m
COLG := \033[32m
COLZ := \033[m

# Multiplatform echo. Interprets escapes. Use it with $(call ECHO,"<message>")
ECHO = printf $(1)"\n"

# Default rule (it's the first one, "all" is just a convention)
all: lib$(PAR_MODULE).rootmap lib$(PAR_MODULE).so
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] $(COLG)PARfile built successfully$(COLZ)")

# Abstract rule to compile classes to objects
%.o: %.cxx %.h
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] compiling $(COLC)$<$(COLZ)")
	@$(CXX) $(PAR_CXXFLAGS) -c $< -o $@

# Generate library from objects
lib$(PAR_MODULE).so: $(OBJS)
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] building shared library")
	@$(LD) $(PAR_SOFLAGS) $^ -o $@ $(EXPLLINKLIBS)

# Generate dictionary (rule broken up to accommodate parallel builds)
G__$(PAR_MODULE).h: G__$(PAR_MODULE).cxx

G__$(PAR_MODULE).cxx: $(HDRS) $(DHDR)
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] generating dictionary")
ifeq ($(ROOT_MAJOR_VERSION),5)
	@rootcint -f $@ -c -p $(ROOT_DEFINES) $(PAR_INCLUDES) $^
endif
ifeq ($(ROOT_MAJOR_VERSION),6)
	@rootcint -f $@ -rmf lib${PAR_MODULE}.rootmap -rml lib${PAR_MODULE} $(ROOT_DEFINES) $(PAR_INCLUDES) $^
endif

# Generate rootmap (for automatic loading of libraries)
lib$(PAR_MODULE).rootmap: lib$(PAR_MODULE).so
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] generating rootmap")
ifeq ($(ROOT_MAJOR_VERSION),5)
	@rlibmap -o $@ -l lib$(PAR_MODULE) -d $(PAR_LIBDEPS) -c $(DHDR)
endif

# Delete all
clean:
	rm -f $(OBJS) *.so G__$(PAR_MODULE).* *.rootmap
