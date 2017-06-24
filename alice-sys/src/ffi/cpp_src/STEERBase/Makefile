# Makefile for STEERBase -- automatically generated

# Include this file from the ROOT installed on the system where we are using the PARfile
include $(shell root-config --etcdir)/Makefile.arch

# Name of this module
PAR_MODULE := STEERBase

# This module depends on the following libraries
PAR_LIBDEPS := libCore libEG libGeom libGpad libGraf3d libGraf libHist libMathCore libMatrix libMinuit libNet libPhysics libRIO libTree libTreePlayer libVMC

# Current OS
OS := $(shell uname)

# ROOT major version
ROOT_MAJOR_VERSION := $(shell root-config --version | cut -f1 -d.)

# Include paths: take into consideration "extra includes" (manually entered by users) and includes
# from other previously enabled PARfiles
PAR_EXTRAINCLUDES = 
# PAR_INCLUDES = -I. $(addprefix -I../,$(ALICE_PARFILES_ENABLED)) $(addprefix -I,$(PAR_EXTRAINCLUDES)) -I$(ALICE_ROOT)/include -I$(ALICE_PHYSICS)/include -I$(shell root-config --incdir)

PAR_INCLUDES = -I. $(addprefix -I../,$(ALICE_PARFILES_ENABLED)) $(addprefix -I,$(PAR_EXTRAINCLUDES)) -I$(shell root-config --incdir)
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

SRCS := AliCentrality.cxx AliCheb2DStack.cxx AliCheb2DStackF.cxx AliCheb2DStackS.cxx AliCheb3DCalc.cxx AliCheb3D.cxx AliCodeTimer.cxx AliCollisionGeometry.cxx AliCounterCollection.cxx AliDAQ.cxx AliDataFile.cxx AliDetectorEventHeader.cxx AliDetectorPID.cxx AliDetectorTagCuts.cxx AliDetectorTag.cxx AliDummyHandler.cxx AliEMCALPIDResponse.cxx AliEventplane.cxx AliEventTagCuts.cxx AliEventTag.cxx AliExternalTrackParam.cxx AliFileTag.cxx AliFileUtilities.cxx AliGenCocktailEventHeader.cxx AliGenDPMjetEventHeader.cxx AliGenEpos3EventHeader.cxx AliGenEposEventHeader.cxx AliGenEventHeader.cxx AliGenEventHeaderTunedPbPb.cxx AliGenGeVSimEventHeader.cxx AliGenHepMCEventHeader.cxx AliGenHerwigEventHeader.cxx AliGenHijingEventHeader.cxx AliGenPythiaEventHeader.cxx AliGenToyEventHeader.cxx AliHeader.cxx AliHLTBaseVGRPAccess.cxx AliHMPIDPIDParams.cxx AliHMPIDPIDResponse.cxx AliInputEventHandler.cxx AliITSPidParams.cxx AliITSPIDResponse.cxx AliLHCTagCuts.cxx AliLHCTag.cxx AliLog.cxx AliMagF.cxx AliMagWrapCheb.cxx AliMathBase.cxx AliMCEvent.cxx AliMCEventHandler.cxx AliMCParticle.cxx AliMCVertex.cxx AliMergeableCollection.cxx AliMixedEvent.cxx AliMultSelectionBase.cxx AliNeutralTrackParam.cxx AliOADBContainer.cxx AliPDG.cxx AliPIDCombined.cxx AliPID.cxx AliPIDResponse.cxx AliPIDValues.cxx AliProdInfo.cxx AliQA.cxx AliRefArray.cxx AliRunTagCuts.cxx AliRunTag.cxx AliStack.cxx AliSysInfo.cxx AliTagCreator.cxx AliTimeStamp.cxx AliTOFHeader.cxx AliTOFPIDParams.cxx AliTOFPIDResponse.cxx AliTOFTriggerMask.cxx AliTPCdEdxInfo.cxx AliTPCPIDResponse.cxx AliTrackReference.cxx AliTRDdEdxParams.cxx AliTRDNDFast.cxx AliTRDPIDParams.cxx AliTRDPIDReference.cxx AliTRDPIDResponse.cxx AliTRDPIDResponseObject.cxx AliTRDTKDInterpolator.cxx AliTriggerScalers.cxx AliTriggerScalersRecord.cxx AliVAD.cxx AliVAODHeader.cxx AliVCaloCells.cxx AliVCaloTrigger.cxx AliVCluster.cxx AliVCuts.cxx AliVersion.cxx AliVEvent.cxx AliVEventHandler.cxx AliVEventPool.cxx AliVfriendEvent.cxx AliVfriendTrack.cxx AliVHeader.cxx AliVMFT.cxx AliVMisc.cxx AliVMultiplicity.cxx AliVParticle.cxx AliVTOFcluster.cxx AliVTOFHit.cxx AliVTOFMatch.cxx AliVTPCseed.cxx AliVTrack.cxx AliVTrdTrack.cxx AliVTrdTracklet.cxx AliVVertex.cxx AliVVZERO.cxx AliVVZEROfriend.cxx AliVZDC.cxx TTreeStream.cxx
HDRS := AliCentrality.h AliCheb2DStack.h AliCheb2DStackF.h AliCheb2DStackS.h AliCheb3DCalc.h AliCheb3D.h AliCodeTimer.h AliCollisionGeometry.h AliCounterCollection.h AliDAQ.h AliDataFile.h AliDetectorEventHeader.h AliDetectorPID.h AliDetectorTagCuts.h AliDetectorTag.h AliDummyHandler.h AliEMCALPIDResponse.h AliEventplane.h AliEventTagCuts.h AliEventTag.h AliExternalTrackParam.h AliFileTag.h AliFileUtilities.h AliGenCocktailEventHeader.h AliGenDPMjetEventHeader.h AliGenEpos3EventHeader.h AliGenEposEventHeader.h AliGenEventHeader.h AliGenEventHeaderTunedPbPb.h AliGenGeVSimEventHeader.h AliGenHepMCEventHeader.h AliGenHerwigEventHeader.h AliGenHijingEventHeader.h AliGenPythiaEventHeader.h AliGenToyEventHeader.h AliHeader.h AliHLTBaseVGRPAccess.h AliHMPIDPIDParams.h AliHMPIDPIDResponse.h AliInputEventHandler.h AliITSPidParams.h AliITSPIDResponse.h AliLHCTagCuts.h AliLHCTag.h AliLog.h AliMagF.h AliMagWrapCheb.h AliMathBase.h AliMCEvent.h AliMCEventHandler.h AliMCParticle.h AliMCVertex.h AliMergeableCollection.h AliMixedEvent.h AliMultSelectionBase.h AliNeutralTrackParam.h AliOADBContainer.h AliPDG.h AliPIDCombined.h AliPID.h AliPIDResponse.h AliPIDValues.h AliProdInfo.h AliQA.h AliRefArray.h AliRunTagCuts.h AliRunTag.h AliStack.h AliSysInfo.h AliTagCreator.h AliTimeStamp.h AliTOFHeader.h AliTOFPIDParams.h AliTOFPIDResponse.h AliTOFTriggerMask.h AliTPCdEdxInfo.h AliTPCPIDResponse.h AliTrackReference.h AliTRDdEdxParams.h AliTRDNDFast.h AliTRDPIDParams.h AliTRDPIDReference.h AliTRDPIDResponse.h AliTRDPIDResponseObject.h AliTRDTKDInterpolator.h AliTriggerScalers.h AliTriggerScalersRecord.h AliVAD.h AliVAODHeader.h AliVCaloCells.h AliVCaloTrigger.h AliVCluster.h AliVCuts.h AliVersion.h AliVEvent.h AliVEventHandler.h AliVEventPool.h AliVfriendEvent.h AliVfriendTrack.h AliVHeader.h AliVMFT.h AliVMisc.h AliVMultiplicity.h AliVParticle.h AliVTOFcluster.h AliVTOFHit.h AliVTOFMatch.h AliVTPCseed.h AliVTrack.h AliVTrdTrack.h AliVTrdTracklet.h AliVVertex.h AliVVZERO.h AliVVZEROfriend.h AliVZDC.h TTreeStream.h AliSharedPtr.h

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
	@echo "$(CXX) $(PAR_CXXFLAGS) -c $< -o $@"
	@$(CXX) $(PAR_CXXFLAGS) -c $< -o $@

# Generate library from objects
lib$(PAR_MODULE).so: $(OBJS)
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] building shared library")
	@echo "$(LD) $(PAR_SOFLAGS) $^ -o $@ $(EXPLLINKLIBS)"
	@$(LD) $(PAR_SOFLAGS) $^ -o $@ $(EXPLLINKLIBS)

# Generate dictionary (rule broken up to accommodate parallel builds)
G__$(PAR_MODULE).h: G__$(PAR_MODULE).cxx

G__$(PAR_MODULE).cxx: $(HDRS) $(DHDR)
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] generating dictionary")
ifeq ($(ROOT_MAJOR_VERSION),5)
	@echo "rootcint -f $@ -c -p $(ROOT_DEFINES) $(PAR_INCLUDES) $^"
	@rootcint -f $@ -c -p $(ROOT_DEFINES) $(PAR_INCLUDES) $^
endif
ifeq ($(ROOT_MAJOR_VERSION),6)
	@rootcint -f $@ -rmf lib${PAR_MODULE}.rootmap -rml lib${PAR_MODULE} $(ROOT_DEFINES) $(PAR_INCLUDES) $^
endif

# Generate rootmap (for automatic loading of libraries)
lib$(PAR_MODULE).rootmap: lib$(PAR_MODULE).so
	@$(call ECHO,"[$(COLM)$(PAR_MODULE)$(COLZ)] generating rootmap")
ifeq ($(ROOT_MAJOR_VERSION),5)
	@echo "rlibmap -o $@ -l lib$(PAR_MODULE) -d $(PAR_LIBDEPS) -c $(DHDR)"
	@rlibmap -o $@ -l lib$(PAR_MODULE) -d $(PAR_LIBDEPS) -c $(DHDR)
endif

# Delete all
clean:
	rm -f $(OBJS) *.so G__$(PAR_MODULE).* *.rootmap
