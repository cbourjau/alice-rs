//////////////////////////////////////////////////////////
// This class has been automatically generated on
// Sun Jun 11 02:46:13 2017 by ROOT version 5.34/30
// from TTree esdTree/Tree with ESD objects
// found on file: /home/christian/Downloads/AliESDs.root
//////////////////////////////////////////////////////////

#ifndef ESD_h
#define ESD_h

// One of the includes seems to enable Weffc++ and wrecks the complete output
// The following line disables this warning on gcc
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Weffc++"

#include <TArrayI.h>
#include <TArrayL64.h>
#include <TObjArray.h>
#include <TRef.h>
#include <TString.h>

class TTree;

/**
 * <div rustbindgen replaces="TString"></div>
 */
template<typename T>
class TString_Simple {
  T* mBuffer;
public:
  const char* Data();
};


/**
 * <div rustbindgen replaces="TArrayI"></div>
 */
template<typename T>
class TArrayI_Simple {
  T* mBuffer;
};

/**
 * <div rustbindgen replaces="TArrayL64"></div>
 */
template<typename T>
class TArrayL64_Simple {
  T* mBuffer;
};

/**
 * <div rustbindgen replaces="TRef"></div>
 */
template<typename T>
class TRef_Simple {
  T* mBuffer;
};

/**
 * <div rustbindgen replaces="TObjArray"></div>
 */
template<typename T>
class TObjArray_Simple {
  T* mBuffer;
};

/**
 * <div rustbindgen replaces="TTree"></div>
 */
template<typename T>
class TTree_Simple {
  T* mBuffer;
};

/**
 * <div rustbindgen replaces="TBranch"></div>
 */
template<typename T>
class TBranch_Simple {
  T* mBuffer;
};

// Fixed size dimensions of array or collections stored in the TTree if any.
const Int_t kMaxAliESDRun = 1;
const Int_t kMaxAliESDHeader = 1;
const Int_t kMaxAliESDZDC = 1;
const Int_t kMaxAliESDFMD = 1;
const Int_t kMaxAliESDVZERO = 1;
const Int_t kMaxAliESDTZERO = 1;
const Int_t kMaxTPCVertex = 1;
const Int_t kMaxSPDVertex = 1;
const Int_t kMaxPrimaryVertex = 1;
const Int_t kMaxAliMultiplicity = 1;
const Int_t kMaxPHOSTrigger = 1;
const Int_t kMaxEMCALTrigger = 1;
const Int_t kMaxSPDPileupVertices = 1;
const Int_t kMaxTrkPileupVertices = 1;
const Int_t kMaxTracks = 50000;
const Int_t kMaxMuonTracks = 44;
const Int_t kMaxMuonClusters = 281;
const Int_t kMaxMuonPads = 2701;
const Int_t kMaxMuonGlobalTracks = 1;
const Int_t kMaxPmdTracks = 1;
const Int_t kMaxAliESDTrdTrigger = 1;
const Int_t kMaxTrdTracks = 525;
const Int_t kMaxTrdTracklets = 46918;
const Int_t kMaxV0s = 81326;
const Int_t kMaxCascades = 1799;
const Int_t kMaxKinks = 383;
const Int_t kMaxCaloClusters = 1592;
const Int_t kMaxEMCALCells = 1;
const Int_t kMaxPHOSCells = 1;
const Int_t kMaxAliRawDataErrorLogs = 16;
const Int_t kMaxAliESDACORDE = 1;
const Int_t kMaxAliESDAD = 1;
const Int_t kMaxAliTOFHeader = 1;
const Int_t kMaxCosmicTracks = 4;
const Int_t kMaxAliESDTOFCluster = 6314;
const Int_t kMaxAliESDTOFHit = 6314;
const Int_t kMaxAliESDTOFMatch = 9831;
const Int_t kMaxAliESDFIT = 1;
const Int_t kMaxHLTGlobalTrigger = 1;
const Int_t kMaxHLTGlobalTrigger_fInputObjectInfo = 1;

class TBranch;

class ESD_t {
public :
  TTree          *fChain;   //!pointer to the analyzed TTree or TChain
  Int_t           fCurrent; //!current Tree number in a TChain
  TFile          *fFile;    //! The file use to create this object

  // Declaration of leaf types
  //AliESDRun       *AliESDRun_;
  UInt_t          AliESDRun_TObject_fUniqueID;
  UInt_t          AliESDRun_TObject_fBits;
  Float_t         AliESDRun_fCurrentL3;
  Float_t         AliESDRun_fCurrentDip;
  Float_t         AliESDRun_fBeamEnergy;
  Double32_t      AliESDRun_fMagneticField;
  Double32_t      AliESDRun_fMeanBeamInt[2][2];
  Double32_t      AliESDRun_fDiamondXY[2];
  Double32_t      AliESDRun_fDiamondCovXY[3];
  Double32_t      AliESDRun_fDiamondZ;
  Double32_t      AliESDRun_fDiamondSig2Z;
  UInt_t          AliESDRun_fPeriodNumber;
  Int_t           AliESDRun_fRunNumber;
  Int_t           AliESDRun_fRecoVersion;
  Int_t           AliESDRun_fBeamParticle[2];
  TString         AliESDRun_fBeamType;
  TObjArray       AliESDRun_fTriggerClasses;
  UInt_t          AliESDRun_fDetInDAQ;
  UInt_t          AliESDRun_fDetInReco;
  Float_t         AliESDRun_fT0spread[4];
  Int_t           AliESDRun_fCaloTriggerType[15];
  Float_t         AliESDRun_fVZEROEqFactors[64];
  Int_t           AliESDRun_fCaloTriggerTypeNew[19];
  UInt_t          AliESDRun_fCTPStart_fUniqueID;
  UInt_t          AliESDRun_fCTPStart_fBits;
  UInt_t          AliESDRun_fCTPStart_fOrbit;
  UInt_t          AliESDRun_fCTPStart_fPeriod;
  ULong64_t       AliESDRun_fCTPStart_fBunchCross;
  //AliESDHeader    *AliESDHeader_;
  UInt_t          AliESDHeader_AliVHeader_fUniqueID;
  UInt_t          AliESDHeader_AliVHeader_fBits;
  TString         AliESDHeader_AliVHeader_fName;
  TString         AliESDHeader_AliVHeader_fTitle;
  ULong64_t       AliESDHeader_fTriggerMask;
  ULong64_t       AliESDHeader_fTriggerMaskNext50;
  UInt_t          AliESDHeader_fOrbitNumber;
  UInt_t          AliESDHeader_fTimeStamp;
  UInt_t          AliESDHeader_fEventType;
  UInt_t          AliESDHeader_fEventSpecie;
  UInt_t          AliESDHeader_fPeriodNumber;
  Int_t           AliESDHeader_fEventNumberInFile;
  UShort_t        AliESDHeader_fBunchCrossNumber;
  UChar_t         AliESDHeader_fTriggerCluster;
  UInt_t          AliESDHeader_fL0TriggerInputs;
  UInt_t          AliESDHeader_fL1TriggerInputs;
  UShort_t        AliESDHeader_fL2TriggerInputs;
  UInt_t          AliESDHeader_fTriggerScalers_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalers_fBits;
  UInt_t          AliESDHeader_fTriggerScalers_fTimestamp_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalers_fTimestamp_fBits;
  UInt_t          AliESDHeader_fTriggerScalers_fTimestamp_fOrbit;
  UInt_t          AliESDHeader_fTriggerScalers_fTimestamp_fPeriod;
  ULong64_t       AliESDHeader_fTriggerScalers_fTimestamp_fBunchCross;
  TObjArray       AliESDHeader_fTriggerScalers_fScalers;
  UInt_t          AliESDHeader_fTriggerScalers_fTimeGroup;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fBits;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fBits;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fOrbit;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fPeriod;
  ULong64_t       AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fBunchCross;
  TObjArray       AliESDHeader_fTriggerScalersDeltaEvent_fScalers;
  UInt_t          AliESDHeader_fTriggerScalersDeltaEvent_fTimeGroup;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fBits;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fUniqueID;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fBits;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fOrbit;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fPeriod;
  ULong64_t       AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fBunchCross;
  TObjArray       AliESDHeader_fTriggerScalersDeltaRun_fScalers;
  UInt_t          AliESDHeader_fTriggerScalersDeltaRun_fTimeGroup;
  TObjArray       AliESDHeader_fTriggerInputsNames;
  TObjArray       AliESDHeader_fIRBufferArray;
  UInt_t          AliESDHeader_fIRInt2InteractionsMap_fUniqueID;
  UInt_t          AliESDHeader_fIRInt2InteractionsMap_fBits;
  UInt_t          AliESDHeader_fIRInt2InteractionsMap_fNbits;
  UInt_t          AliESDHeader_fIRInt2InteractionsMap_fNbytes;
  UChar_t         AliESDHeader_fIRInt2InteractionsMap_fAllBits[1];   //[AliESDHeader.fIRInt2InteractionsMap.fNbytes]
  UInt_t          AliESDHeader_fIRInt1InteractionsMap_fUniqueID;
  UInt_t          AliESDHeader_fIRInt1InteractionsMap_fBits;
  UInt_t          AliESDHeader_fIRInt1InteractionsMap_fNbits;
  UInt_t          AliESDHeader_fIRInt1InteractionsMap_fNbytes;
  UChar_t         AliESDHeader_fIRInt1InteractionsMap_fAllBits[1];   //[AliESDHeader.fIRInt1InteractionsMap.fNbytes]
  UChar_t         AliESDHeader_fTPCNoiseFilterCounter[3];
  //AliESDZDC       *AliESDZDC_;
  UInt_t          AliESDZDC_AliVZDC_fUniqueID;
  UInt_t          AliESDZDC_AliVZDC_fBits;
  Double32_t      AliESDZDC_fZDCN1Energy;
  Double32_t      AliESDZDC_fZDCP1Energy;
  Double32_t      AliESDZDC_fZDCN2Energy;
  Double32_t      AliESDZDC_fZDCP2Energy;
  Double32_t      AliESDZDC_fZDCEMEnergy;
  Double32_t      AliESDZDC_fZDCEMEnergy1;
  Double32_t      AliESDZDC_fZN1TowerEnergy[5];
  Double32_t      AliESDZDC_fZN2TowerEnergy[5];
  Double32_t      AliESDZDC_fZP1TowerEnergy[5];
  Double32_t      AliESDZDC_fZP2TowerEnergy[5];
  Double32_t      AliESDZDC_fZN1TowerEnergyLR[5];
  Double32_t      AliESDZDC_fZN2TowerEnergyLR[5];
  Double32_t      AliESDZDC_fZP1TowerEnergyLR[5];
  Double32_t      AliESDZDC_fZP2TowerEnergyLR[5];
  Short_t         AliESDZDC_fZDCParticipants;
  Short_t         AliESDZDC_fZDCPartSideA;
  Short_t         AliESDZDC_fZDCPartSideC;
  Double32_t      AliESDZDC_fImpactParameter;
  Double32_t      AliESDZDC_fImpactParamSideA;
  Double32_t      AliESDZDC_fImpactParamSideC;
  Double32_t      AliESDZDC_fZNACentrCoord[2];
  Double32_t      AliESDZDC_fZNCCentrCoord[2];
  UInt_t          AliESDZDC_fESDQuality;
  UInt_t          AliESDZDC_fVMEScaler[32];
  Int_t           AliESDZDC_fZDCTDCData[32][4];
  Float_t         AliESDZDC_fZDCTDCCorrected[32][4];
  Bool_t          AliESDZDC_fZNCTDChit;
  Bool_t          AliESDZDC_fZNATDChit;
  Bool_t          AliESDZDC_fZPCTDChit;
  Bool_t          AliESDZDC_fZPATDChit;
  Bool_t          AliESDZDC_fZEM1TDChit;
  Bool_t          AliESDZDC_fZEM2TDChit;
  Int_t           AliESDZDC_fZDCTDCChannels[7];
  //AliESDFMD       *AliESDFMD_;
  UInt_t          AliESDFMD_TObject_fUniqueID;
  UInt_t          AliESDFMD_TObject_fBits;
  UInt_t          AliESDFMD_fMultiplicity_fUniqueID;
  UInt_t          AliESDFMD_fMultiplicity_fBits;
  UShort_t        AliESDFMD_fMultiplicity_fMaxDetectors;
  UShort_t        AliESDFMD_fMultiplicity_fMaxRings;
  UShort_t        AliESDFMD_fMultiplicity_fMaxSectors;
  UShort_t        AliESDFMD_fMultiplicity_fMaxStrips;
  Int_t           AliESDFMD_fMultiplicity_fTotal;
  Float_t         AliESDFMD_fMultiplicity_fData[51200];   //[AliESDFMD.fMultiplicity.fTotal]
  UInt_t          AliESDFMD_fEta_fUniqueID;
  UInt_t          AliESDFMD_fEta_fBits;
  UShort_t        AliESDFMD_fEta_fMaxDetectors;
  UShort_t        AliESDFMD_fEta_fMaxRings;
  UShort_t        AliESDFMD_fEta_fMaxSectors;
  UShort_t        AliESDFMD_fEta_fMaxStrips;
  Int_t           AliESDFMD_fEta_fTotal;
  Float_t         AliESDFMD_fEta_fData[3072];   //[AliESDFMD.fEta.fTotal]
  Float_t         AliESDFMD_fNoiseFactor;
  Bool_t          AliESDFMD_fAngleCorrected;
  //AliESDVZERO     *AliESDVZERO_;
  UInt_t          AliESDVZERO_AliVVZERO_fUniqueID;
  UInt_t          AliESDVZERO_AliVVZERO_fBits;
  UInt_t          AliESDVZERO_fBBtriggerV0A;
  UInt_t          AliESDVZERO_fBGtriggerV0A;
  UInt_t          AliESDVZERO_fBBtriggerV0C;
  UInt_t          AliESDVZERO_fBGtriggerV0C;
  Float_t         AliESDVZERO_fMultiplicity[64];
  Float_t         AliESDVZERO_fAdc[64];
  Float_t         AliESDVZERO_fTime[64];
  Float_t         AliESDVZERO_fWidth[64];
  Bool_t          AliESDVZERO_fBBFlag[64];
  Bool_t          AliESDVZERO_fBGFlag[64];
  Float_t         AliESDVZERO_fV0ATime;
  Float_t         AliESDVZERO_fV0CTime;
  Float_t         AliESDVZERO_fV0ATimeError;
  Float_t         AliESDVZERO_fV0CTimeError;
  Int_t           AliESDVZERO_fV0ADecision;
  Int_t           AliESDVZERO_fV0CDecision;
  UShort_t        AliESDVZERO_fTriggerChargeA;
  UShort_t        AliESDVZERO_fTriggerChargeC;
  UShort_t        AliESDVZERO_fTriggerBits;
  Bool_t          AliESDVZERO_fIsBB[64][21];
  Bool_t          AliESDVZERO_fIsBG[64][21];
  //AliESDTZERO     *AliESDTZERO_;
  UInt_t          AliESDTZERO_TObject_fUniqueID;
  UInt_t          AliESDTZERO_TObject_fBits;
  Float_t         AliESDTZERO_fT0clock;
  Double32_t      AliESDTZERO_fT0TOF[3];
  Double32_t      AliESDTZERO_fT0zVertex;
  Double32_t      AliESDTZERO_fT0timeStart;
  Int_t           AliESDTZERO_fT0trig;
  Double32_t      AliESDTZERO_fT0time[24];
  Double32_t      AliESDTZERO_fT0amplitude[24];
  Float_t         AliESDTZERO_fTimeFull[24][5];
  Float_t         AliESDTZERO_fOrA[5];
  Float_t         AliESDTZERO_fOrC[5];
  Float_t         AliESDTZERO_fTVDC[5];
  Bool_t          AliESDTZERO_fPileup;
  Bool_t          AliESDTZERO_fSattelite;
  Float_t         AliESDTZERO_fMultC;
  Float_t         AliESDTZERO_fMultA;
  Bool_t          AliESDTZERO_fBackground;
  Float_t         AliESDTZERO_fPileupTime[6];
  Double32_t      AliESDTZERO_fT0TOFbest[3];
  Double32_t      AliESDTZERO_fT0NewAmplitude[24];
  UInt_t          AliESDTZERO_fPileupBits_fUniqueID;
  UInt_t          AliESDTZERO_fPileupBits_fBits;
  UInt_t          AliESDTZERO_fPileupBits_fNbits;
  UInt_t          AliESDTZERO_fPileupBits_fNbytes;
  UChar_t         AliESDTZERO_fPileupBits_fAllBits[1];   //[AliESDTZERO.fPileupBits.fNbytes]
  //AliESDVertex    *TPCVertex_;
  UInt_t          TPCVertex_AliVertex_fUniqueID;
  UInt_t          TPCVertex_AliVertex_fBits;
  TString         TPCVertex_AliVertex_fName;
  TString         TPCVertex_AliVertex_fTitle;
  Double32_t      TPCVertex_AliVertex_fPosition[3];
  Double32_t      TPCVertex_AliVertex_fSigma;
  Int_t           TPCVertex_AliVertex_fNContributors;
  Int_t           TPCVertex_AliVertex_fNIndices;
  UShort_t        TPCVertex_AliVertex_fIndices[4153];   //[TPCVertex.AliVertex.fNIndices]
  Double32_t      TPCVertex_fCovXX;
  Double32_t      TPCVertex_fCovXY;
  Double32_t      TPCVertex_fCovYY;
  Double32_t      TPCVertex_fCovXZ;
  Double32_t      TPCVertex_fCovYZ;
  Double32_t      TPCVertex_fCovZZ;
  Double32_t      TPCVertex_fSNR[3];
  Double32_t      TPCVertex_fChi2;
  Char_t          TPCVertex_fID;
  Char_t          TPCVertex_fBCID;
  //AliESDVertex    *SPDVertex_;
  UInt_t          SPDVertex_AliVertex_fUniqueID;
  UInt_t          SPDVertex_AliVertex_fBits;
  TString         SPDVertex_AliVertex_fName;
  TString         SPDVertex_AliVertex_fTitle;
  Double32_t      SPDVertex_AliVertex_fPosition[3];
  Double32_t      SPDVertex_AliVertex_fSigma;
  Int_t           SPDVertex_AliVertex_fNContributors;
  Int_t           SPDVertex_AliVertex_fNIndices;
  UShort_t        SPDVertex_AliVertex_fIndices[1];   //[SPDVertex.AliVertex.fNIndices]
  Double32_t      SPDVertex_fCovXX;
  Double32_t      SPDVertex_fCovXY;
  Double32_t      SPDVertex_fCovYY;
  Double32_t      SPDVertex_fCovXZ;
  Double32_t      SPDVertex_fCovYZ;
  Double32_t      SPDVertex_fCovZZ;
  Double32_t      SPDVertex_fSNR[3];
  Double32_t      SPDVertex_fChi2;
  Char_t          SPDVertex_fID;
  Char_t          SPDVertex_fBCID;
  //AliESDVertex    *PrimaryVertex_;
  UInt_t          PrimaryVertex_AliVertex_fUniqueID;
  UInt_t          PrimaryVertex_AliVertex_fBits;
  TString         PrimaryVertex_AliVertex_fName;
  TString         PrimaryVertex_AliVertex_fTitle;
  Double32_t      PrimaryVertex_AliVertex_fPosition[3];
  Double32_t      PrimaryVertex_AliVertex_fSigma;
  Int_t           PrimaryVertex_AliVertex_fNContributors;
  Int_t           PrimaryVertex_AliVertex_fNIndices;
  UShort_t        PrimaryVertex_AliVertex_fIndices[3309];   //[PrimaryVertex.AliVertex.fNIndices]
  Double32_t      PrimaryVertex_fCovXX;
  Double32_t      PrimaryVertex_fCovXY;
  Double32_t      PrimaryVertex_fCovYY;
  Double32_t      PrimaryVertex_fCovXZ;
  Double32_t      PrimaryVertex_fCovYZ;
  Double32_t      PrimaryVertex_fCovZZ;
  Double32_t      PrimaryVertex_fSNR[3];
  Double32_t      PrimaryVertex_fChi2;
  Char_t          PrimaryVertex_fID;
  Char_t          PrimaryVertex_fBCID;
  //AliMultiplicity *AliMultiplicity_;
  UInt_t          AliMultiplicity_AliVMultiplicity_fUniqueID;
  UInt_t          AliMultiplicity_AliVMultiplicity_fBits;
  TString         AliMultiplicity_AliVMultiplicity_fName;
  TString         AliMultiplicity_AliVMultiplicity_fTitle;
  Int_t           AliMultiplicity_fNtracks;
  Int_t           AliMultiplicity_fNsingle;
  Int_t           AliMultiplicity_fNsingleSPD2;
  Float_t         AliMultiplicity_fDPhiWindow2;
  Float_t         AliMultiplicity_fDThetaWindow2;
  Float_t         AliMultiplicity_fDPhiShift;
  Float_t         AliMultiplicity_fNStdDev;
  Int_t           AliMultiplicity_fLabels[5129];   //[AliMultiplicity.fNtracks]
  Int_t           AliMultiplicity_fLabelsL2[5129];   //[AliMultiplicity.fNtracks]
  UInt_t          AliMultiplicity_fUsedClusS[4321];   //[AliMultiplicity.fNsingle]
  ULong64_t       AliMultiplicity_fUsedClusT[5129];   //[AliMultiplicity.fNtracks]
  Double32_t      AliMultiplicity_fTh[5129];   //[AliMultiplicity.fNtracks]
  Double32_t      AliMultiplicity_fPhi[5129];   //[AliMultiplicity.fNtracks]
  Double32_t      AliMultiplicity_fDeltTh[5129];   //[AliMultiplicity.fNtracks]
  Double32_t      AliMultiplicity_fDeltPhi[5129];   //[AliMultiplicity.fNtracks]
  Double32_t      AliMultiplicity_fThsingle[4321];   //[AliMultiplicity.fNsingle]
  Double32_t      AliMultiplicity_fPhisingle[4321];   //[AliMultiplicity.fNsingle]
  Int_t           AliMultiplicity_fLabelssingle[4321];   //[AliMultiplicity.fNsingle]
  Short_t         AliMultiplicity_fFiredChips[2];
  UInt_t          AliMultiplicity_fITSClusters[6];
  UInt_t          AliMultiplicity_fFastOrFiredChips_fUniqueID;
  UInt_t          AliMultiplicity_fFastOrFiredChips_fBits;
  UInt_t          AliMultiplicity_fFastOrFiredChips_fNbits;
  UInt_t          AliMultiplicity_fFastOrFiredChips_fNbytes;
  UChar_t         AliMultiplicity_fFastOrFiredChips_fAllBits[1];   //[AliMultiplicity.fFastOrFiredChips.fNbytes]
  UInt_t          AliMultiplicity_fClusterFiredChips_fUniqueID;
  UInt_t          AliMultiplicity_fClusterFiredChips_fBits;
  UInt_t          AliMultiplicity_fClusterFiredChips_fNbits;
  UInt_t          AliMultiplicity_fClusterFiredChips_fNbytes;
  UChar_t         AliMultiplicity_fClusterFiredChips_fAllBits[1];   //[AliMultiplicity.fClusterFiredChips.fNbytes]
  //AliESDCaloTrigger *PHOSTrigger_;
  UInt_t          PHOSTrigger_AliVCaloTrigger_fUniqueID;
  UInt_t          PHOSTrigger_AliVCaloTrigger_fBits;
  TString         PHOSTrigger_AliVCaloTrigger_fName;
  TString         PHOSTrigger_AliVCaloTrigger_fTitle;
  Int_t           PHOSTrigger_fNEntries;
  Int_t           PHOSTrigger_fCurrent;
  Int_t           PHOSTrigger_fColumn[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fRow[1];   //[PHOSTrigger.fNEntries]
  Float_t         PHOSTrigger_fAmplitude[1];   //[PHOSTrigger.fNEntries]
  Float_t         PHOSTrigger_fTime[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fNL0Times[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fL1TimeSum[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fTriggerBits[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fL1Threshold[4];
  Int_t           PHOSTrigger_fL1V0[2];
  Int_t           PHOSTrigger_fL1FrameMask;
  Int_t           PHOSTrigger_fL1DCALThreshold[4];
  Int_t           PHOSTrigger_fL1SubRegion[1];   //[PHOSTrigger.fNEntries]
  Int_t           PHOSTrigger_fL1DCALFrameMask;
  Int_t           PHOSTrigger_fMedian[2];
  Int_t           PHOSTrigger_fTriggerBitWord;
  Int_t           PHOSTrigger_fL1DCALV0[2];
  //AliESDCaloTrigger *EMCALTrigger_;
  UInt_t          EMCALTrigger_AliVCaloTrigger_fUniqueID;
  UInt_t          EMCALTrigger_AliVCaloTrigger_fBits;
  TString         EMCALTrigger_AliVCaloTrigger_fName;
  TString         EMCALTrigger_AliVCaloTrigger_fTitle;
  Int_t           EMCALTrigger_fNEntries;
  Int_t           EMCALTrigger_fCurrent;
  Int_t           EMCALTrigger_fColumn[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fRow[2611];   //[EMCALTrigger.fNEntries]
  Float_t         EMCALTrigger_fAmplitude[2611];   //[EMCALTrigger.fNEntries]
  Float_t         EMCALTrigger_fTime[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fNL0Times[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fL1TimeSum[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fTriggerBits[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fL1Threshold[4];
  Int_t           EMCALTrigger_fL1V0[2];
  Int_t           EMCALTrigger_fL1FrameMask;
  Int_t           EMCALTrigger_fL1DCALThreshold[4];
  Int_t           EMCALTrigger_fL1SubRegion[2611];   //[EMCALTrigger.fNEntries]
  Int_t           EMCALTrigger_fL1DCALFrameMask;
  Int_t           EMCALTrigger_fMedian[2];
  Int_t           EMCALTrigger_fTriggerBitWord;
  Int_t           EMCALTrigger_fL1DCALV0[2];
  Int_t           SPDPileupVertices_;
  UInt_t          SPDPileupVertices_fUniqueID[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  UInt_t          SPDPileupVertices_fBits[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  TString         SPDPileupVertices_fName[kMaxSPDPileupVertices];
  TString         SPDPileupVertices_fTitle[kMaxSPDPileupVertices];
  Double32_t      SPDPileupVertices_fPosition[kMaxSPDPileupVertices][3];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fSigma[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Int_t           SPDPileupVertices_fNContributors[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Int_t           SPDPileupVertices_fNIndices[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  UShort_t       *SPDPileupVertices_fIndices[kMaxSPDPileupVertices];   //[SPDPileupVertices_fNIndices]
  Double32_t      SPDPileupVertices_fCovXX[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fCovXY[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fCovYY[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fCovXZ[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fCovYZ[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fCovZZ[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fSNR[kMaxSPDPileupVertices][3];   //[SPDPileupVertices_]
  Double32_t      SPDPileupVertices_fChi2[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Char_t          SPDPileupVertices_fID[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Char_t          SPDPileupVertices_fBCID[kMaxSPDPileupVertices];   //[SPDPileupVertices_]
  Int_t           TrkPileupVertices_;
  UInt_t          TrkPileupVertices_fUniqueID[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  UInt_t          TrkPileupVertices_fBits[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  TString         TrkPileupVertices_fName[kMaxTrkPileupVertices];
  TString         TrkPileupVertices_fTitle[kMaxTrkPileupVertices];
  Double32_t      TrkPileupVertices_fPosition[kMaxTrkPileupVertices][3];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fSigma[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Int_t           TrkPileupVertices_fNContributors[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Int_t           TrkPileupVertices_fNIndices[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  UShort_t       *TrkPileupVertices_fIndices[kMaxTrkPileupVertices];   //[TrkPileupVertices_fNIndices]
  Double32_t      TrkPileupVertices_fCovXX[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fCovXY[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fCovYY[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fCovXZ[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fCovYZ[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fCovZZ[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fSNR[kMaxTrkPileupVertices][3];   //[TrkPileupVertices_]
  Double32_t      TrkPileupVertices_fChi2[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Char_t          TrkPileupVertices_fID[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Char_t          TrkPileupVertices_fBCID[kMaxTrkPileupVertices];   //[TrkPileupVertices_]
  Int_t           Tracks_;
  UInt_t          Tracks_fUniqueID[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fBits[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fX[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fAlpha[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fP[kMaxTracks][5];   //[Tracks_]
  Double32_t      Tracks_fC[kMaxTracks][15];   //[Tracks_]
  UInt_t          Tracks_fTPCFitMap_fUniqueID[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCFitMap_fBits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCFitMap_fNbits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCFitMap_fNbytes[kMaxTracks];   //[Tracks_]
  UChar_t        *Tracks_fTPCFitMap_fAllBits[kMaxTracks];   //[Tracks_fTPCFitMap_fNbytes]
  UInt_t          Tracks_fTPCClusterMap_fUniqueID[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCClusterMap_fBits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCClusterMap_fNbits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCClusterMap_fNbytes[kMaxTracks];   //[Tracks_]
  UChar_t        *Tracks_fTPCClusterMap_fAllBits[kMaxTracks];   //[Tracks_fTPCClusterMap_fNbytes]
  UInt_t          Tracks_fTPCSharedMap_fUniqueID[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCSharedMap_fBits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCSharedMap_fNbits[kMaxTracks];   //[Tracks_]
  UInt_t          Tracks_fTPCSharedMap_fNbytes[kMaxTracks];   //[Tracks_]
  UChar_t        *Tracks_fTPCSharedMap_fAllBits[kMaxTracks];   //[Tracks_fTPCSharedMap_fNbytes]
  UShort_t        Tracks_fFrTrackID[kMaxTracks];   //[Tracks_]
  ULong_t         Tracks_fFlags[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fID[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fLabel[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fITSLabel[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fITSModule[kMaxTracks][12];   //[Tracks_]
  Int_t           Tracks_fTPCLabel[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fTRDLabel[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fTOFindex[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fHMPIDqn[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fHMPIDcluIdx[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fCaloIndex[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fKinkIndexes[kMaxTracks][3];   //[Tracks_]
  Int_t           Tracks_fV0Indexes[kMaxTracks][3];   //[Tracks_]
  Double32_t      Tracks_fHMPIDtrkTheta[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDtrkPhi[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDsignal[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fdTPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fzTPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCddTPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCdzTPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCzzTPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCchi2TPC[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fD[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fZ[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCdd[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCdz[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCzz[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fITSchi2Std[kMaxTracks][3];   //[Tracks_]
  Double32_t      Tracks_fITSchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTPCchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTPCchi2Iter1[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTRDchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTOFchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDchi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fGlobalChi2[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fITSsignal[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fITSdEdxSamples[kMaxTracks][4];   //[Tracks_]
  Double32_t      Tracks_fTPCsignal[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTPCsignalS[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTPCPoints[kMaxTracks][4];   //[Tracks_]
  Double32_t      Tracks_fTRDsignal[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTRDQuality[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fTRDBudget[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCaloDx[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fCaloDz[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDtrkX[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDtrkY[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDmipX[kMaxTracks];   //[Tracks_]
  Double32_t      Tracks_fHMPIDmipY[kMaxTracks];   //[Tracks_]
  UShort_t        Tracks_fTPCncls[kMaxTracks];   //[Tracks_]
  UShort_t        Tracks_fTPCnclsF[kMaxTracks];   //[Tracks_]
  UShort_t        Tracks_fTPCsignalN[kMaxTracks];   //[Tracks_]
  UShort_t        Tracks_fTPCnclsIter1[kMaxTracks];   //[Tracks_]
  UShort_t        Tracks_fTPCnclsFIter1[kMaxTracks];   //[Tracks_]
  Char_t          Tracks_fITSncls[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fITSClusterMap[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fITSSharedMap[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fTRDncls[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fTRDncls0[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fTRDntracklets[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fTRDNchamberdEdx[kMaxTracks];   //[Tracks_]
  UChar_t         Tracks_fTRDNclusterdEdx[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fTRDnSlices[kMaxTracks];   //[Tracks_]
  Double32_t     *Tracks_fTRDslices[kMaxTracks];   //[Tracks_fTRDnSlices]
  Char_t          Tracks_fTRDTimBin[kMaxTracks][6];   //[Tracks_]
  Char_t          Tracks_fVertexID[kMaxTracks];   //[Tracks_]
  Char_t          Tracks_fPIDForTracking[kMaxTracks];   //[Tracks_]
  Double_t        Tracks_fTrackPhiOnEMCal[kMaxTracks];   //[Tracks_]
  Double_t        Tracks_fTrackEtaOnEMCal[kMaxTracks];   //[Tracks_]
  Double_t        Tracks_fTrackPtOnEMCal[kMaxTracks];   //[Tracks_]
  Int_t           Tracks_fNtofClusters[kMaxTracks];   //[Tracks_]
  Int_t          *Tracks_fTOFcluster[kMaxTracks];   //[Tracks_fNtofClusters]
  Int_t           MuonTracks_;
  UInt_t          MuonTracks_fUniqueID[kMaxMuonTracks];   //[MuonTracks_]
  UInt_t          MuonTracks_fBits[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fInverseBendingMomentum[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaX[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaY[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fZ[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fBendingCoor[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fNonBendingCoor[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fInverseBendingMomentumAtDCA[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaXAtDCA[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaYAtDCA[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fBendingCoorAtDCA[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fNonBendingCoorAtDCA[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fInverseBendingMomentumUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaXUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fThetaYUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fZUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fBendingCoorUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fNonBendingCoorUncorrected[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fCovariances[kMaxMuonTracks][15];   //[MuonTracks_]
  Double32_t      MuonTracks_fRAtAbsorberEnd[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fChi2[kMaxMuonTracks];   //[MuonTracks_]
  Double32_t      MuonTracks_fChi2MatchTrigger[kMaxMuonTracks];   //[MuonTracks_]
  Int_t           MuonTracks_fLocalTrigger[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fX1Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fY1Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fX2Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fY2Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fX3Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fY3Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fX4Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fY4Pattern[kMaxMuonTracks];   //[MuonTracks_]
  UInt_t          MuonTracks_fMuonClusterMap[kMaxMuonTracks];   //[MuonTracks_]
  UShort_t        MuonTracks_fHitsPatternInTrigCh[kMaxMuonTracks];   //[MuonTracks_]
  UInt_t          MuonTracks_fHitsPatternInTrigChTrk[kMaxMuonTracks];   //[MuonTracks_]
  UChar_t         MuonTracks_fNHit[kMaxMuonTracks];   //[MuonTracks_]
  Int_t           MuonTracks_fLabel[kMaxMuonTracks];   //[MuonTracks_]
  Int_t           MuonClusters_;
  UInt_t          MuonClusters_fUniqueID[kMaxMuonClusters];   //[MuonClusters_]
  UInt_t          MuonClusters_fBits[kMaxMuonClusters];   //[MuonClusters_]
  Double32_t      MuonClusters_fXYZ[kMaxMuonClusters][3];   //[MuonClusters_]
  Double32_t      MuonClusters_fErrXY[kMaxMuonClusters][2];   //[MuonClusters_]
  Double32_t      MuonClusters_fCharge[kMaxMuonClusters];   //[MuonClusters_]
  Double32_t      MuonClusters_fChi2[kMaxMuonClusters];   //[MuonClusters_]
  Int_t           MuonClusters_fNPads[kMaxMuonClusters];   //[MuonClusters_]
  Int_t           MuonClusters_fLabel[kMaxMuonClusters];   //[MuonClusters_]
  Int_t           MuonPads_;
  UInt_t          MuonPads_fUniqueID[kMaxMuonPads];   //[MuonPads_]
  UInt_t          MuonPads_fBits[kMaxMuonPads];   //[MuonPads_]
  Int_t           MuonPads_fADC[kMaxMuonPads];   //[MuonPads_]
  Double32_t      MuonPads_fCharge[kMaxMuonPads];   //[MuonPads_]
  Int_t           MuonGlobalTracks_;
  UInt_t          MuonGlobalTracks_fUniqueID[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  UInt_t          MuonGlobalTracks_fBits[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Short_t         MuonGlobalTracks_fCharge[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Short_t         MuonGlobalTracks_fMatchTrigger[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Short_t         MuonGlobalTracks_fNMFTClusters[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Short_t         MuonGlobalTracks_fNWrongMFTClustersMC[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  ULong_t         MuonGlobalTracks_fMFTClusterPattern[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fPx[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fPy[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fPz[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fPt[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fP[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fEta[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fRapidity[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fFirstTrackingPointX[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fFirstTrackingPointY[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fFirstTrackingPointZ[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fXAtVertex[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fYAtVertex[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fRAtAbsorberEnd[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fChi2OverNdf[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fChi2MatchTrigger[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Int_t           MuonGlobalTracks_fLabel[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  UInt_t          MuonGlobalTracks_fMuonClusterMap[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  UShort_t        MuonGlobalTracks_fHitsPatternInTrigCh[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  UInt_t          MuonGlobalTracks_fHitsPatternInTrigChTrk[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Int_t           MuonGlobalTracks_fLoCircuit[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Bool_t          MuonGlobalTracks_fIsConnected[kMaxMuonGlobalTracks];   //[MuonGlobalTracks_]
  Double_t        MuonGlobalTracks_fProdVertexXYZ[kMaxMuonGlobalTracks][3];   //[MuonGlobalTracks_]
  Int_t           PmdTracks_;
  UInt_t          PmdTracks_fUniqueID[kMaxPmdTracks];   //[PmdTracks_]
  UInt_t          PmdTracks_fBits[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fX[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fY[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fZ[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fCluADC[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fCluPID[kMaxPmdTracks];   //[PmdTracks_]
  UChar_t         PmdTracks_fDet[kMaxPmdTracks];   //[PmdTracks_]
  UChar_t         PmdTracks_fNcell[kMaxPmdTracks];   //[PmdTracks_]
  Int_t           PmdTracks_fSmn[kMaxPmdTracks];   //[PmdTracks_]
  Int_t           PmdTracks_fTrackNo[kMaxPmdTracks];   //[PmdTracks_]
  Int_t           PmdTracks_fTrackPid[kMaxPmdTracks];   //[PmdTracks_]
  UShort_t        PmdTracks_fClMatching[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fSigX[kMaxPmdTracks];   //[PmdTracks_]
  Double32_t      PmdTracks_fSigY[kMaxPmdTracks];   //[PmdTracks_]
  //AliESDTrdTrigger *AliESDTrdTrigger_;
  UInt_t          AliESDTrdTrigger_TObject_fUniqueID;
  UInt_t          AliESDTrdTrigger_TObject_fBits;
  UInt_t          AliESDTrdTrigger_fFlags[18];
  Int_t           TrdTracks_;
  UInt_t          TrdTracks_fUniqueID[kMaxTrdTracks];   //[TrdTracks_]
  UInt_t          TrdTracks_fBits[kMaxTrdTracks];   //[TrdTracks_]
  Int_t           TrdTracks_fSector[kMaxTrdTracks];   //[TrdTracks_]
  Char_t          TrdTracks_fStack[kMaxTrdTracks];   //[TrdTracks_]
  Int_t           TrdTracks_fA[kMaxTrdTracks];   //[TrdTracks_]
  Int_t           TrdTracks_fB[kMaxTrdTracks];   //[TrdTracks_]
  Short_t         TrdTracks_fC[kMaxTrdTracks];   //[TrdTracks_]
  Short_t         TrdTracks_fY[kMaxTrdTracks];   //[TrdTracks_]
  UChar_t         TrdTracks_fPID[kMaxTrdTracks];   //[TrdTracks_]
  Char_t          TrdTracks_fLayerMask[kMaxTrdTracks];   //[TrdTracks_]
  Char_t          TrdTracks_fTrackletIndex[kMaxTrdTracks][6];   //[TrdTracks_]
  UShort_t        TrdTracks_fFlags[kMaxTrdTracks];   //[TrdTracks_]
  UChar_t         TrdTracks_fFlagsTiming[kMaxTrdTracks];   //[TrdTracks_]
  UChar_t         TrdTracks_fReserved[kMaxTrdTracks];   //[TrdTracks_]
  TRef            TrdTracks_fTrackletRefs[6][kMaxTrdTracks];
  TRef            TrdTracks_fTrackMatch[kMaxTrdTracks];
  Int_t           TrdTracks_fLabel[kMaxTrdTracks];   //[TrdTracks_]
  Int_t           TrdTracklets_;
  UInt_t          TrdTracklets_fUniqueID[kMaxTrdTracklets];   //[TrdTracklets_]
  UInt_t          TrdTracklets_fBits[kMaxTrdTracklets];   //[TrdTracklets_]
  Short_t         TrdTracklets_fHCId[kMaxTrdTracklets];   //[TrdTracklets_]
  UInt_t          TrdTracklets_fTrackletWord[kMaxTrdTracklets];   //[TrdTracklets_]
  Int_t           TrdTracklets_fLabel[kMaxTrdTracklets];   //[TrdTracklets_]
  Int_t           V0s_;
  UInt_t          V0s_fUniqueID[kMaxV0s];   //[V0s_]
  UInt_t          V0s_fBits[kMaxV0s];   //[V0s_]
  UInt_t          V0s_fParamN_fUniqueID[kMaxV0s];   //[V0s_]
  UInt_t          V0s_fParamN_fBits[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamN_fX[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamN_fAlpha[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamN_fP[kMaxV0s][5];   //[V0s_]
  Double32_t      V0s_fParamN_fC[kMaxV0s][15];   //[V0s_]
  UInt_t          V0s_fParamP_fUniqueID[kMaxV0s];   //[V0s_]
  UInt_t          V0s_fParamP_fBits[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamP_fX[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamP_fAlpha[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fParamP_fP[kMaxV0s][5];   //[V0s_]
  Double32_t      V0s_fParamP_fC[kMaxV0s][15];   //[V0s_]
  Double32_t      V0s_fEffMass[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fDcaV0Daughters[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fChi2V0[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fPos[kMaxV0s][3];   //[V0s_]
  Double32_t      V0s_fPosCov[kMaxV0s][6];   //[V0s_]
  Double32_t      V0s_fNmom[kMaxV0s][3];   //[V0s_]
  Double32_t      V0s_fPmom[kMaxV0s][3];   //[V0s_]
  Double32_t      V0s_fNormDCAPrim[kMaxV0s][2];   //[V0s_]
  Double32_t      V0s_fRr[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fDistSigma[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fChi2Before[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fChi2After[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fCausality[kMaxV0s][4];   //[V0s_]
  Double32_t      V0s_fAngle[kMaxV0s][3];   //[V0s_]
  Double32_t      V0s_fPointAngleFi[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fPointAngleTh[kMaxV0s];   //[V0s_]
  Double32_t      V0s_fPointAngle[kMaxV0s];   //[V0s_]
  Int_t           V0s_fPdgCode[kMaxV0s];   //[V0s_]
  Int_t           V0s_fNidx[kMaxV0s];   //[V0s_]
  Int_t           V0s_fPidx[kMaxV0s];   //[V0s_]
  Short_t         V0s_fStatus[kMaxV0s];   //[V0s_]
  Short_t         V0s_fNBefore[kMaxV0s];   //[V0s_]
  Short_t         V0s_fNAfter[kMaxV0s];   //[V0s_]
  Bool_t          V0s_fOnFlyStatus[kMaxV0s];   //[V0s_]
  Int_t           Cascades_;
  UInt_t          Cascades_fUniqueID[kMaxCascades];   //[Cascades_]
  UInt_t          Cascades_fBits[kMaxCascades];   //[Cascades_]
  UInt_t          Cascades_fParamN_fUniqueID[kMaxCascades];   //[Cascades_]
  UInt_t          Cascades_fParamN_fBits[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamN_fX[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamN_fAlpha[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamN_fP[kMaxCascades][5];   //[Cascades_]
  Double32_t      Cascades_fParamN_fC[kMaxCascades][15];   //[Cascades_]
  UInt_t          Cascades_fParamP_fUniqueID[kMaxCascades];   //[Cascades_]
  UInt_t          Cascades_fParamP_fBits[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamP_fX[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamP_fAlpha[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fParamP_fP[kMaxCascades][5];   //[Cascades_]
  Double32_t      Cascades_fParamP_fC[kMaxCascades][15];   //[Cascades_]
  Double32_t      Cascades_fEffMass[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fDcaV0Daughters[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fChi2V0[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fPos[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fPosCov[kMaxCascades][6];   //[Cascades_]
  Double32_t      Cascades_fNmom[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fPmom[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fNormDCAPrim[kMaxCascades][2];   //[Cascades_]
  Double32_t      Cascades_fRr[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fDistSigma[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fChi2Before[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fChi2After[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fCausality[kMaxCascades][4];   //[Cascades_]
  Double32_t      Cascades_fAngle[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fPointAngleFi[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fPointAngleTh[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fPointAngle[kMaxCascades];   //[Cascades_]
  Int_t           Cascades_fPdgCode[kMaxCascades];   //[Cascades_]
  Int_t           Cascades_fNidx[kMaxCascades];   //[Cascades_]
  Int_t           Cascades_fPidx[kMaxCascades];   //[Cascades_]
  Short_t         Cascades_fStatus[kMaxCascades];   //[Cascades_]
  Short_t         Cascades_fNBefore[kMaxCascades];   //[Cascades_]
  Short_t         Cascades_fNAfter[kMaxCascades];   //[Cascades_]
  Bool_t          Cascades_fOnFlyStatus[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fEffMassXi[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fChi2Xi[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fDcaXiDaughters[kMaxCascades];   //[Cascades_]
  Double32_t      Cascades_fPosXi[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fPosCovXi[kMaxCascades][6];   //[Cascades_]
  Double32_t      Cascades_fBachMom[kMaxCascades][3];   //[Cascades_]
  Double32_t      Cascades_fBachMomCov[kMaxCascades][6];   //[Cascades_]
  Int_t           Cascades_fPdgCodeXi[kMaxCascades];   //[Cascades_]
  Int_t           Cascades_fBachIdx[kMaxCascades];   //[Cascades_]
  Int_t           Kinks_;
  UInt_t          Kinks_fUniqueID[kMaxKinks];   //[Kinks_]
  UInt_t          Kinks_fBits[kMaxKinks];   //[Kinks_]
  UInt_t          Kinks_fParamDaughter_fUniqueID[kMaxKinks];   //[Kinks_]
  UInt_t          Kinks_fParamDaughter_fBits[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamDaughter_fX[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamDaughter_fAlpha[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamDaughter_fP[kMaxKinks][5];   //[Kinks_]
  Double32_t      Kinks_fParamDaughter_fC[kMaxKinks][15];   //[Kinks_]
  UInt_t          Kinks_fParamMother_fUniqueID[kMaxKinks];   //[Kinks_]
  UInt_t          Kinks_fParamMother_fBits[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamMother_fX[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamMother_fAlpha[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fParamMother_fP[kMaxKinks][5];   //[Kinks_]
  Double32_t      Kinks_fParamMother_fC[kMaxKinks][15];   //[Kinks_]
  Double32_t      Kinks_fDist1[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fDist2[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fPdr[kMaxKinks][3];   //[Kinks_]
  Double32_t      Kinks_fXr[kMaxKinks][3];   //[Kinks_]
  Double32_t      Kinks_fPm[kMaxKinks][3];   //[Kinks_]
  Double32_t      Kinks_fRr[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fShapeFactor[kMaxKinks];   //[Kinks_]
  Double32_t      Kinks_fTPCdensity[kMaxKinks][2][2];   //[Kinks_]
  Double32_t      Kinks_fAngle[kMaxKinks][3];   //[Kinks_]
  Int_t           Kinks_fLab[kMaxKinks][2];   //[Kinks_]
  Int_t           Kinks_fIndex[kMaxKinks][2];   //[Kinks_]
  Short_t         Kinks_fID[kMaxKinks];   //[Kinks_]
  UChar_t         Kinks_fRow0[kMaxKinks];   //[Kinks_]
  UChar_t         Kinks_fMultiple[kMaxKinks][2];   //[Kinks_]
  UChar_t         Kinks_fTPCncls[kMaxKinks][2];   //[Kinks_]
  Char_t          Kinks_fStatus[kMaxKinks][12];   //[Kinks_]
  Int_t           CaloClusters_;
  UInt_t          CaloClusters_fUniqueID[kMaxCaloClusters];   //[CaloClusters_]
  UInt_t          CaloClusters_fBits[kMaxCaloClusters];   //[CaloClusters_]
  Int_t           CaloClusters_fNCells[kMaxCaloClusters];   //[CaloClusters_]
  UShort_t       *CaloClusters_fCellsAbsId[kMaxCaloClusters];   //[CaloClusters_fNCells]
  Double32_t     *CaloClusters_fCellsAmpFraction[kMaxCaloClusters];   //[CaloClusters_fNCells]
  Double32_t      CaloClusters_fGlobalPos[kMaxCaloClusters][3];   //[CaloClusters_]
  Double32_t      CaloClusters_fEnergy[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fDispersion[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fChi2[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fM20[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fM02[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fEmcCpvDistance[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fTrackDx[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fTrackDz[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fDistToBadChannel[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fPID[kMaxCaloClusters][14];   //[CaloClusters_]
  Int_t           CaloClusters_fID[kMaxCaloClusters];   //[CaloClusters_]
  UChar_t         CaloClusters_fNExMax[kMaxCaloClusters];   //[CaloClusters_]
  Char_t          CaloClusters_fClusterType[kMaxCaloClusters];   //[CaloClusters_]
  Double_t        CaloClusters_fTOF[kMaxCaloClusters];   //[CaloClusters_]
  Double32_t      CaloClusters_fCoreEnergy[kMaxCaloClusters];   //[CaloClusters_]
  UInt_t          CaloClusters_fNLabel[kMaxCaloClusters];   //[CaloClusters_]
  UShort_t       *CaloClusters_fClusterMCEdepFraction[kMaxCaloClusters];   //[CaloClusters_fNLabel]
  UInt_t         *CaloClusters_fCellsMCEdepFractionMap[kMaxCaloClusters];   //[CaloClusters_fNCells]
  //AliESDCaloCells *EMCALCells_;
  UInt_t          EMCALCells_AliVCaloCells_fUniqueID;
  UInt_t          EMCALCells_AliVCaloCells_fBits;
  TString         EMCALCells_AliVCaloCells_fName;
  TString         EMCALCells_AliVCaloCells_fTitle;
  Int_t           EMCALCells_fNCells;
  Bool_t          EMCALCells_fHGLG[5504];   //[EMCALCells.fNCells]
  Short_t         EMCALCells_fCellNumber[5504];   //[EMCALCells.fNCells]
  Double32_t      EMCALCells_fAmplitude[5504];   //[EMCALCells.fNCells]
  Double32_t      EMCALCells_fTime[5504];   //[EMCALCells.fNCells]
  Double32_t      EMCALCells_fEFraction[5504];   //[EMCALCells.fNCells]
  Int_t           EMCALCells_fMCLabel[5504];   //[EMCALCells.fNCells]
  Char_t          EMCALCells_fType;
  //AliESDCaloCells *PHOSCells_;
  UInt_t          PHOSCells_AliVCaloCells_fUniqueID;
  UInt_t          PHOSCells_AliVCaloCells_fBits;
  TString         PHOSCells_AliVCaloCells_fName;
  TString         PHOSCells_AliVCaloCells_fTitle;
  Int_t           PHOSCells_fNCells;
  Bool_t          PHOSCells_fHGLG[1];   //[PHOSCells.fNCells]
  Short_t         PHOSCells_fCellNumber[1];   //[PHOSCells.fNCells]
  Double32_t      PHOSCells_fAmplitude[1];   //[PHOSCells.fNCells]
  Double32_t      PHOSCells_fTime[1];   //[PHOSCells.fNCells]
  Double32_t      PHOSCells_fEFraction[1];   //[PHOSCells.fNCells]
  Int_t           PHOSCells_fMCLabel[1];   //[PHOSCells.fNCells]
  Char_t          PHOSCells_fType;
  Int_t           AliRawDataErrorLogs_;
  UInt_t          AliRawDataErrorLogs_fUniqueID[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  UInt_t          AliRawDataErrorLogs_fBits[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  TString         AliRawDataErrorLogs_fName[kMaxAliRawDataErrorLogs];
  TString         AliRawDataErrorLogs_fTitle[kMaxAliRawDataErrorLogs];
  Int_t           AliRawDataErrorLogs_fEventNumber[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  Int_t           AliRawDataErrorLogs_fDdlID[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  Int_t           AliRawDataErrorLogs_fErrorLevel[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  Int_t           AliRawDataErrorLogs_fErrorCode[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  Int_t           AliRawDataErrorLogs_fCount[kMaxAliRawDataErrorLogs];   //[AliRawDataErrorLogs_]
  //AliESDACORDE    *AliESDACORDE_;
  UInt_t          AliESDACORDE_TObject_fUniqueID;
  UInt_t          AliESDACORDE_TObject_fBits;
  Bool_t          AliESDACORDE_fACORDEBitPattern[60];
  //AliESDAD        *AliESDAD_;
  UInt_t          AliESDAD_AliVAD_fUniqueID;
  UInt_t          AliESDAD_AliVAD_fBits;
  UInt_t          AliESDAD_fBBtriggerADA;
  UInt_t          AliESDAD_fBGtriggerADA;
  UInt_t          AliESDAD_fBBtriggerADC;
  UInt_t          AliESDAD_fBGtriggerADC;
  Float_t         AliESDAD_fMultiplicity[16];
  Float_t         AliESDAD_fAdc[16];
  Float_t         AliESDAD_fTime[16];
  Float_t         AliESDAD_fWidth[16];
  Bool_t          AliESDAD_fBBFlag[16];
  Bool_t          AliESDAD_fBGFlag[16];
  Float_t         AliESDAD_fADATime;
  Float_t         AliESDAD_fADCTime;
  Float_t         AliESDAD_fADATimeError;
  Float_t         AliESDAD_fADCTimeError;
  Int_t           AliESDAD_fADADecision;
  Int_t           AliESDAD_fADCDecision;
  UShort_t        AliESDAD_fTriggerChargeA;
  UShort_t        AliESDAD_fTriggerChargeC;
  UShort_t        AliESDAD_fTriggerBits;
  Bool_t          AliESDAD_fIsBB[16][21];
  Bool_t          AliESDAD_fIsBG[16][21];
  Float_t         AliESDAD_fAdcTail[16];
  Float_t         AliESDAD_fAdcTrigger[16];
  //AliTOFHeader    *AliTOFHeader_;
  UInt_t          AliTOFHeader_TObject_fUniqueID;
  UInt_t          AliTOFHeader_TObject_fBits;
  Float_t         AliTOFHeader_fDefaultEventTimeValue;
  Float_t         AliTOFHeader_fDefaultEventTimeRes;
  Int_t           AliTOFHeader_fNbins;
  Float_t         AliTOFHeader_fTOFtimeResolution;
  Float_t         AliTOFHeader_fT0spread;
  Int_t           AliTOFHeader_fNumberOfTOFclusters;
  Int_t           AliTOFHeader_fNumberOfTOFtrgPads;
  Int_t           CosmicTracks_;
  UInt_t          CosmicTracks_fUniqueID[kMaxCosmicTracks];   //[CosmicTracks_]
  UInt_t          CosmicTracks_fBits[kMaxCosmicTracks];   //[CosmicTracks_]
  Double32_t      CosmicTracks_fX[kMaxCosmicTracks];   //[CosmicTracks_]
  Double32_t      CosmicTracks_fAlpha[kMaxCosmicTracks];   //[CosmicTracks_]
  Double32_t      CosmicTracks_fP[kMaxCosmicTracks][5];   //[CosmicTracks_]
  Double32_t      CosmicTracks_fC[kMaxCosmicTracks][15];   //[CosmicTracks_]
  Int_t           CosmicTracks_fESDtrackIndex[kMaxCosmicTracks][2];   //[CosmicTracks_]
  Int_t           CosmicTracks_fNCluster[kMaxCosmicTracks];   //[CosmicTracks_]
  Double_t        CosmicTracks_fLeverArm[kMaxCosmicTracks];   //[CosmicTracks_]
  Double_t        CosmicTracks_fChi2PerCluster[kMaxCosmicTracks];   //[CosmicTracks_]
  Double_t        CosmicTracks_fImpactD[kMaxCosmicTracks];   //[CosmicTracks_]
  Double_t        CosmicTracks_fImpactZ[kMaxCosmicTracks];   //[CosmicTracks_]
  Bool_t          CosmicTracks_fIsReuse[kMaxCosmicTracks];   //[CosmicTracks_]
  Double_t        CosmicTracks_fFindableRatio[kMaxCosmicTracks];   //[CosmicTracks_]
  Int_t           AliESDTOFCluster_;
  UInt_t          AliESDTOFCluster_fUniqueID[kMaxAliESDTOFCluster];   //[AliESDTOFCluster_]
  UInt_t          AliESDTOFCluster_fBits[kMaxAliESDTOFCluster];   //[AliESDTOFCluster_]
  Int_t           AliESDTOFCluster_fID[kMaxAliESDTOFCluster];   //[AliESDTOFCluster_]
  Char_t          AliESDTOFCluster_fNTOFhits[kMaxAliESDTOFCluster];   //[AliESDTOFCluster_]
  Char_t          AliESDTOFCluster_fNmatchableTracks[kMaxAliESDTOFCluster];   //[AliESDTOFCluster_]
  Int_t           AliESDTOFCluster_fHitIndex[kMaxAliESDTOFCluster][4];   //[AliESDTOFCluster_]
  Int_t           AliESDTOFCluster_fMatchIndex[kMaxAliESDTOFCluster][7];   //[AliESDTOFCluster_]
  Int_t           AliESDTOFHit_;
  UInt_t          AliESDTOFHit_fUniqueID[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  UInt_t          AliESDTOFHit_fBits[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Double32_t      AliESDTOFHit_fTimeRaw[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Double32_t      AliESDTOFHit_fTime[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Double32_t      AliESDTOFHit_fTOT[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Int_t           AliESDTOFHit_fTOFLabel[kMaxAliESDTOFHit][3];   //[AliESDTOFHit_]
  Int_t           AliESDTOFHit_fL0L1Latency[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Int_t           AliESDTOFHit_fDeltaBC[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Int_t           AliESDTOFHit_fTOFchannel[kMaxAliESDTOFHit];   //[AliESDTOFHit_]
  Int_t           AliESDTOFMatch_;
  UInt_t          AliESDTOFMatch_fUniqueID[kMaxAliESDTOFMatch];   //[AliESDTOFMatch_]
  UInt_t          AliESDTOFMatch_fBits[kMaxAliESDTOFMatch];   //[AliESDTOFMatch_]
  Double32_t      AliESDTOFMatch_fDx[kMaxAliESDTOFMatch];   //[AliESDTOFMatch_]
  Double32_t      AliESDTOFMatch_fDz[kMaxAliESDTOFMatch];   //[AliESDTOFMatch_]
  Double32_t      AliESDTOFMatch_fTrackLength[kMaxAliESDTOFMatch];   //[AliESDTOFMatch_]
  Double32_t      AliESDTOFMatch_fIntegratedTimes[kMaxAliESDTOFMatch][9];   //[AliESDTOFMatch_]
  //AliESDFIT       *AliESDFIT_;
  UInt_t          AliESDFIT_TObject_fUniqueID;
  UInt_t          AliESDFIT_TObject_fBits;
  Float_t         AliESDFIT_fT0[3];
  Float_t         AliESDFIT_fFITzVertex;
  Float_t         AliESDFIT_fFITtime[240];
  Float_t         AliESDFIT_fFITamplitude[240];
  Float_t         AliESDFIT_fT0best[3];
  //AliESDHLTDecision *HLTGlobalTrigger_;
  UInt_t          HLTGlobalTrigger_TNamed_fUniqueID;
  UInt_t          HLTGlobalTrigger_TNamed_fBits;
  TString         HLTGlobalTrigger_TNamed_fName;
  TString         HLTGlobalTrigger_TNamed_fTitle;
  Int_t           HLTGlobalTrigger_fInputObjectInfo_;
  UInt_t          HLTGlobalTrigger_fInputObjectInfo_fUniqueID[kMaxHLTGlobalTrigger_fInputObjectInfo];   //[HLTGlobalTrigger.fInputObjectInfo_]
  UInt_t          HLTGlobalTrigger_fInputObjectInfo_fBits[kMaxHLTGlobalTrigger_fInputObjectInfo];   //[HLTGlobalTrigger.fInputObjectInfo_]
  TString         HLTGlobalTrigger_fInputObjectInfo_fName[kMaxHLTGlobalTrigger_fInputObjectInfo];
  TString         HLTGlobalTrigger_fInputObjectInfo_fTitle[kMaxHLTGlobalTrigger_fInputObjectInfo];
  TArrayI         HLTGlobalTrigger_fTriggerItems;
  TArrayL64       HLTGlobalTrigger_fCounters;
  ULong64_t       fDetectorStatus;
  UInt_t          fDAQDetectorPattern;
  UInt_t          fDAQAttributes;
  Int_t           fNTPCClusters;

  // List of branches
  TBranch        *b_AliESDRun_TObject_fUniqueID;   //!
  TBranch        *b_AliESDRun_TObject_fBits;   //!
  TBranch        *b_AliESDRun_fCurrentL3;   //!
  TBranch        *b_AliESDRun_fCurrentDip;   //!
  TBranch        *b_AliESDRun_fBeamEnergy;   //!
  TBranch        *b_AliESDRun_fMagneticField;   //!
  TBranch        *b_AliESDRun_fMeanBeamInt;   //!
  TBranch        *b_AliESDRun_fDiamondXY;   //!
  TBranch        *b_AliESDRun_fDiamondCovXY;   //!
  TBranch        *b_AliESDRun_fDiamondZ;   //!
  TBranch        *b_AliESDRun_fDiamondSig2Z;   //!
  TBranch        *b_AliESDRun_fPeriodNumber;   //!
  TBranch        *b_AliESDRun_fRunNumber;   //!
  TBranch        *b_AliESDRun_fRecoVersion;   //!
  TBranch        *b_AliESDRun_fBeamParticle;   //!
  TBranch        *b_AliESDRun_fBeamType;   //!
  TBranch        *b_AliESDRun_fTriggerClasses;   //!
  TBranch        *b_AliESDRun_fDetInDAQ;   //!
  TBranch        *b_AliESDRun_fDetInReco;   //!
  TBranch        *b_AliESDRun_fT0spread;   //!
  TBranch        *b_AliESDRun_fCaloTriggerType;   //!
  TBranch        *b_AliESDRun_fVZEROEqFactors;   //!
  TBranch        *b_AliESDRun_fCaloTriggerTypeNew;   //!
  TBranch        *b_AliESDRun_fCTPStart_fUniqueID;   //!
  TBranch        *b_AliESDRun_fCTPStart_fBits;   //!
  TBranch        *b_AliESDRun_fCTPStart_fOrbit;   //!
  TBranch        *b_AliESDRun_fCTPStart_fPeriod;   //!
  TBranch        *b_AliESDRun_fCTPStart_fBunchCross;   //!
  TBranch        *b_AliESDHeader_AliVHeader_fUniqueID;   //!
  TBranch        *b_AliESDHeader_AliVHeader_fBits;   //!
  TBranch        *b_AliESDHeader_AliVHeader_fName;   //!
  TBranch        *b_AliESDHeader_AliVHeader_fTitle;   //!
  TBranch        *b_AliESDHeader_fTriggerMask;   //!
  TBranch        *b_AliESDHeader_fTriggerMaskNext50;   //!
  TBranch        *b_AliESDHeader_fOrbitNumber;   //!
  TBranch        *b_AliESDHeader_fTimeStamp;   //!
  TBranch        *b_AliESDHeader_fEventType;   //!
  TBranch        *b_AliESDHeader_fEventSpecie;   //!
  TBranch        *b_AliESDHeader_fPeriodNumber;   //!
  TBranch        *b_AliESDHeader_fEventNumberInFile;   //!
  TBranch        *b_AliESDHeader_fBunchCrossNumber;   //!
  TBranch        *b_AliESDHeader_fTriggerCluster;   //!
  TBranch        *b_AliESDHeader_fL0TriggerInputs;   //!
  TBranch        *b_AliESDHeader_fL1TriggerInputs;   //!
  TBranch        *b_AliESDHeader_fL2TriggerInputs;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimestamp_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimestamp_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimestamp_fOrbit;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimestamp_fPeriod;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimestamp_fBunchCross;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fScalers;   //!
  TBranch        *b_AliESDHeader_fTriggerScalers_fTimeGroup;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fOrbit;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fPeriod;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimestamp_fBunchCross;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fScalers;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaEvent_fTimeGroup;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fBits;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fOrbit;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fPeriod;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimestamp_fBunchCross;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fScalers;   //!
  TBranch        *b_AliESDHeader_fTriggerScalersDeltaRun_fTimeGroup;   //!
  TBranch        *b_AliESDHeader_fTriggerInputsNames;   //!
  TBranch        *b_AliESDHeader_fIRBufferArray;   //!
  TBranch        *b_AliESDHeader_fIRInt2InteractionsMap_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fIRInt2InteractionsMap_fBits;   //!
  TBranch        *b_AliESDHeader_fIRInt2InteractionsMap_fNbits;   //!
  TBranch        *b_AliESDHeader_fIRInt2InteractionsMap_fNbytes;   //!
  TBranch        *b_AliESDHeader_fIRInt2InteractionsMap_fAllBits;   //!
  TBranch        *b_AliESDHeader_fIRInt1InteractionsMap_fUniqueID;   //!
  TBranch        *b_AliESDHeader_fIRInt1InteractionsMap_fBits;   //!
  TBranch        *b_AliESDHeader_fIRInt1InteractionsMap_fNbits;   //!
  TBranch        *b_AliESDHeader_fIRInt1InteractionsMap_fNbytes;   //!
  TBranch        *b_AliESDHeader_fIRInt1InteractionsMap_fAllBits;   //!
  TBranch        *b_AliESDHeader_fTPCNoiseFilterCounter;   //!
  TBranch        *b_AliESDZDC_AliVZDC_fUniqueID;   //!
  TBranch        *b_AliESDZDC_AliVZDC_fBits;   //!
  TBranch        *b_AliESDZDC_fZDCN1Energy;   //!
  TBranch        *b_AliESDZDC_fZDCP1Energy;   //!
  TBranch        *b_AliESDZDC_fZDCN2Energy;   //!
  TBranch        *b_AliESDZDC_fZDCP2Energy;   //!
  TBranch        *b_AliESDZDC_fZDCEMEnergy;   //!
  TBranch        *b_AliESDZDC_fZDCEMEnergy1;   //!
  TBranch        *b_AliESDZDC_fZN1TowerEnergy;   //!
  TBranch        *b_AliESDZDC_fZN2TowerEnergy;   //!
  TBranch        *b_AliESDZDC_fZP1TowerEnergy;   //!
  TBranch        *b_AliESDZDC_fZP2TowerEnergy;   //!
  TBranch        *b_AliESDZDC_fZN1TowerEnergyLR;   //!
  TBranch        *b_AliESDZDC_fZN2TowerEnergyLR;   //!
  TBranch        *b_AliESDZDC_fZP1TowerEnergyLR;   //!
  TBranch        *b_AliESDZDC_fZP2TowerEnergyLR;   //!
  TBranch        *b_AliESDZDC_fZDCParticipants;   //!
  TBranch        *b_AliESDZDC_fZDCPartSideA;   //!
  TBranch        *b_AliESDZDC_fZDCPartSideC;   //!
  TBranch        *b_AliESDZDC_fImpactParameter;   //!
  TBranch        *b_AliESDZDC_fImpactParamSideA;   //!
  TBranch        *b_AliESDZDC_fImpactParamSideC;   //!
  TBranch        *b_AliESDZDC_fZNACentrCoord;   //!
  TBranch        *b_AliESDZDC_fZNCCentrCoord;   //!
  TBranch        *b_AliESDZDC_fESDQuality;   //!
  TBranch        *b_AliESDZDC_fVMEScaler;   //!
  TBranch        *b_AliESDZDC_fZDCTDCData;   //!
  TBranch        *b_AliESDZDC_fZDCTDCCorrected;   //!
  TBranch        *b_AliESDZDC_fZNCTDChit;   //!
  TBranch        *b_AliESDZDC_fZNATDChit;   //!
  TBranch        *b_AliESDZDC_fZPCTDChit;   //!
  TBranch        *b_AliESDZDC_fZPATDChit;   //!
  TBranch        *b_AliESDZDC_fZEM1TDChit;   //!
  TBranch        *b_AliESDZDC_fZEM2TDChit;   //!
  TBranch        *b_AliESDZDC_fZDCTDCChannels;   //!
  TBranch        *b_AliESDFMD_TObject_fUniqueID;   //!
  TBranch        *b_AliESDFMD_TObject_fBits;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fUniqueID;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fBits;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fMaxDetectors;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fMaxRings;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fMaxSectors;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fMaxStrips;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fTotal;   //!
  TBranch        *b_AliESDFMD_fMultiplicity_fData;   //!
  TBranch        *b_AliESDFMD_fEta_fUniqueID;   //!
  TBranch        *b_AliESDFMD_fEta_fBits;   //!
  TBranch        *b_AliESDFMD_fEta_fMaxDetectors;   //!
  TBranch        *b_AliESDFMD_fEta_fMaxRings;   //!
  TBranch        *b_AliESDFMD_fEta_fMaxSectors;   //!
  TBranch        *b_AliESDFMD_fEta_fMaxStrips;   //!
  TBranch        *b_AliESDFMD_fEta_fTotal;   //!
  TBranch        *b_AliESDFMD_fEta_fData;   //!
  TBranch        *b_AliESDFMD_fNoiseFactor;   //!
  TBranch        *b_AliESDFMD_fAngleCorrected;   //!
  TBranch        *b_AliESDVZERO_AliVVZERO_fUniqueID;   //!
  TBranch        *b_AliESDVZERO_AliVVZERO_fBits;   //!
  TBranch        *b_AliESDVZERO_fBBtriggerV0A;   //!
  TBranch        *b_AliESDVZERO_fBGtriggerV0A;   //!
  TBranch        *b_AliESDVZERO_fBBtriggerV0C;   //!
  TBranch        *b_AliESDVZERO_fBGtriggerV0C;   //!
  TBranch        *b_AliESDVZERO_fMultiplicity;   //!
  TBranch        *b_AliESDVZERO_fAdc;   //!
  TBranch        *b_AliESDVZERO_fTime;   //!
  TBranch        *b_AliESDVZERO_fWidth;   //!
  TBranch        *b_AliESDVZERO_fBBFlag;   //!
  TBranch        *b_AliESDVZERO_fBGFlag;   //!
  TBranch        *b_AliESDVZERO_fV0ATime;   //!
  TBranch        *b_AliESDVZERO_fV0CTime;   //!
  TBranch        *b_AliESDVZERO_fV0ATimeError;   //!
  TBranch        *b_AliESDVZERO_fV0CTimeError;   //!
  TBranch        *b_AliESDVZERO_fV0ADecision;   //!
  TBranch        *b_AliESDVZERO_fV0CDecision;   //!
  TBranch        *b_AliESDVZERO_fTriggerChargeA;   //!
  TBranch        *b_AliESDVZERO_fTriggerChargeC;   //!
  TBranch        *b_AliESDVZERO_fTriggerBits;   //!
  TBranch        *b_AliESDVZERO_fIsBB;   //!
  TBranch        *b_AliESDVZERO_fIsBG;   //!
  TBranch        *b_AliESDTZERO_TObject_fUniqueID;   //!
  TBranch        *b_AliESDTZERO_TObject_fBits;   //!
  TBranch        *b_AliESDTZERO_fT0clock;   //!
  TBranch        *b_AliESDTZERO_fT0TOF;   //!
  TBranch        *b_AliESDTZERO_fT0zVertex;   //!
  TBranch        *b_AliESDTZERO_fT0timeStart;   //!
  TBranch        *b_AliESDTZERO_fT0trig;   //!
  TBranch        *b_AliESDTZERO_fT0time;   //!
  TBranch        *b_AliESDTZERO_fT0amplitude;   //!
  TBranch        *b_AliESDTZERO_fTimeFull;   //!
  TBranch        *b_AliESDTZERO_fOrA;   //!
  TBranch        *b_AliESDTZERO_fOrC;   //!
  TBranch        *b_AliESDTZERO_fTVDC;   //!
  TBranch        *b_AliESDTZERO_fPileup;   //!
  TBranch        *b_AliESDTZERO_fSattelite;   //!
  TBranch        *b_AliESDTZERO_fMultC;   //!
  TBranch        *b_AliESDTZERO_fMultA;   //!
  TBranch        *b_AliESDTZERO_fBackground;   //!
  TBranch        *b_AliESDTZERO_fPileupTime;   //!
  TBranch        *b_AliESDTZERO_fT0TOFbest;   //!
  TBranch        *b_AliESDTZERO_fT0NewAmplitude;   //!
  TBranch        *b_AliESDTZERO_fPileupBits_fUniqueID;   //!
  TBranch        *b_AliESDTZERO_fPileupBits_fBits;   //!
  TBranch        *b_AliESDTZERO_fPileupBits_fNbits;   //!
  TBranch        *b_AliESDTZERO_fPileupBits_fNbytes;   //!
  TBranch        *b_AliESDTZERO_fPileupBits_fAllBits;   //!
  TBranch        *b_TPCVertex_AliVertex_fUniqueID;   //!
  TBranch        *b_TPCVertex_AliVertex_fBits;   //!
  TBranch        *b_TPCVertex_AliVertex_fName;   //!
  TBranch        *b_TPCVertex_AliVertex_fTitle;   //!
  TBranch        *b_TPCVertex_AliVertex_fPosition;   //!
  TBranch        *b_TPCVertex_AliVertex_fSigma;   //!
  TBranch        *b_TPCVertex_AliVertex_fNContributors;   //!
  TBranch        *b_TPCVertex_AliVertex_fNIndices;   //!
  TBranch        *b_TPCVertex_AliVertex_fIndices;   //!
  TBranch        *b_TPCVertex_fCovXX;   //!
  TBranch        *b_TPCVertex_fCovXY;   //!
  TBranch        *b_TPCVertex_fCovYY;   //!
  TBranch        *b_TPCVertex_fCovXZ;   //!
  TBranch        *b_TPCVertex_fCovYZ;   //!
  TBranch        *b_TPCVertex_fCovZZ;   //!
  TBranch        *b_TPCVertex_fSNR;   //!
  TBranch        *b_TPCVertex_fChi2;   //!
  TBranch        *b_TPCVertex_fID;   //!
  TBranch        *b_TPCVertex_fBCID;   //!
  TBranch        *b_SPDVertex_AliVertex_fUniqueID;   //!
  TBranch        *b_SPDVertex_AliVertex_fBits;   //!
  TBranch        *b_SPDVertex_AliVertex_fName;   //!
  TBranch        *b_SPDVertex_AliVertex_fTitle;   //!
  TBranch        *b_SPDVertex_AliVertex_fPosition;   //!
  TBranch        *b_SPDVertex_AliVertex_fSigma;   //!
  TBranch        *b_SPDVertex_AliVertex_fNContributors;   //!
  TBranch        *b_SPDVertex_AliVertex_fNIndices;   //!
  TBranch        *b_SPDVertex_AliVertex_fIndices;   //!
  TBranch        *b_SPDVertex_fCovXX;   //!
  TBranch        *b_SPDVertex_fCovXY;   //!
  TBranch        *b_SPDVertex_fCovYY;   //!
  TBranch        *b_SPDVertex_fCovXZ;   //!
  TBranch        *b_SPDVertex_fCovYZ;   //!
  TBranch        *b_SPDVertex_fCovZZ;   //!
  TBranch        *b_SPDVertex_fSNR;   //!
  TBranch        *b_SPDVertex_fChi2;   //!
  TBranch        *b_SPDVertex_fID;   //!
  TBranch        *b_SPDVertex_fBCID;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fUniqueID;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fBits;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fName;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fTitle;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fPosition;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fSigma;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fNContributors;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fNIndices;   //!
  TBranch        *b_PrimaryVertex_AliVertex_fIndices;   //!
  TBranch        *b_PrimaryVertex_fCovXX;   //!
  TBranch        *b_PrimaryVertex_fCovXY;   //!
  TBranch        *b_PrimaryVertex_fCovYY;   //!
  TBranch        *b_PrimaryVertex_fCovXZ;   //!
  TBranch        *b_PrimaryVertex_fCovYZ;   //!
  TBranch        *b_PrimaryVertex_fCovZZ;   //!
  TBranch        *b_PrimaryVertex_fSNR;   //!
  TBranch        *b_PrimaryVertex_fChi2;   //!
  TBranch        *b_PrimaryVertex_fID;   //!
  TBranch        *b_PrimaryVertex_fBCID;   //!
  TBranch        *b_AliMultiplicity_AliVMultiplicity_fUniqueID;   //!
  TBranch        *b_AliMultiplicity_AliVMultiplicity_fBits;   //!
  TBranch        *b_AliMultiplicity_AliVMultiplicity_fName;   //!
  TBranch        *b_AliMultiplicity_AliVMultiplicity_fTitle;   //!
  TBranch        *b_AliMultiplicity_fNtracks;   //!
  TBranch        *b_AliMultiplicity_fNsingle;   //!
  TBranch        *b_AliMultiplicity_fNsingleSPD2;   //!
  TBranch        *b_AliMultiplicity_fDPhiWindow2;   //!
  TBranch        *b_AliMultiplicity_fDThetaWindow2;   //!
  TBranch        *b_AliMultiplicity_fDPhiShift;   //!
  TBranch        *b_AliMultiplicity_fNStdDev;   //!
  TBranch        *b_AliMultiplicity_fLabels;   //!
  TBranch        *b_AliMultiplicity_fLabelsL2;   //!
  TBranch        *b_AliMultiplicity_fUsedClusS;   //!
  TBranch        *b_AliMultiplicity_fUsedClusT;   //!
  TBranch        *b_AliMultiplicity_fTh;   //!
  TBranch        *b_AliMultiplicity_fPhi;   //!
  TBranch        *b_AliMultiplicity_fDeltTh;   //!
  TBranch        *b_AliMultiplicity_fDeltPhi;   //!
  TBranch        *b_AliMultiplicity_fThsingle;   //!
  TBranch        *b_AliMultiplicity_fPhisingle;   //!
  TBranch        *b_AliMultiplicity_fLabelssingle;   //!
  TBranch        *b_AliMultiplicity_fFiredChips;   //!
  TBranch        *b_AliMultiplicity_fITSClusters;   //!
  TBranch        *b_AliMultiplicity_fFastOrFiredChips_fUniqueID;   //!
  TBranch        *b_AliMultiplicity_fFastOrFiredChips_fBits;   //!
  TBranch        *b_AliMultiplicity_fFastOrFiredChips_fNbits;   //!
  TBranch        *b_AliMultiplicity_fFastOrFiredChips_fNbytes;   //!
  TBranch        *b_AliMultiplicity_fFastOrFiredChips_fAllBits;   //!
  TBranch        *b_AliMultiplicity_fClusterFiredChips_fUniqueID;   //!
  TBranch        *b_AliMultiplicity_fClusterFiredChips_fBits;   //!
  TBranch        *b_AliMultiplicity_fClusterFiredChips_fNbits;   //!
  TBranch        *b_AliMultiplicity_fClusterFiredChips_fNbytes;   //!
  TBranch        *b_AliMultiplicity_fClusterFiredChips_fAllBits;   //!
  TBranch        *b_PHOSTrigger_AliVCaloTrigger_fUniqueID;   //!
  TBranch        *b_PHOSTrigger_AliVCaloTrigger_fBits;   //!
  TBranch        *b_PHOSTrigger_AliVCaloTrigger_fName;   //!
  TBranch        *b_PHOSTrigger_AliVCaloTrigger_fTitle;   //!
  TBranch        *b_PHOSTrigger_fNEntries;   //!
  TBranch        *b_PHOSTrigger_fCurrent;   //!
  TBranch        *b_PHOSTrigger_fColumn;   //!
  TBranch        *b_PHOSTrigger_fRow;   //!
  TBranch        *b_PHOSTrigger_fAmplitude;   //!
  TBranch        *b_PHOSTrigger_fTime;   //!
  TBranch        *b_PHOSTrigger_fNL0Times;   //!
  TBranch        *b_PHOSTrigger_fL1TimeSum;   //!
  TBranch        *b_PHOSTrigger_fTriggerBits;   //!
  TBranch        *b_PHOSTrigger_fL1Threshold;   //!
  TBranch        *b_PHOSTrigger_fL1V0;   //!
  TBranch        *b_PHOSTrigger_fL1FrameMask;   //!
  TBranch        *b_PHOSTrigger_fL1DCALThreshold;   //!
  TBranch        *b_PHOSTrigger_fL1SubRegion;   //!
  TBranch        *b_PHOSTrigger_fL1DCALFrameMask;   //!
  TBranch        *b_PHOSTrigger_fMedian;   //!
  TBranch        *b_PHOSTrigger_fTriggerBitWord;   //!
  TBranch        *b_PHOSTrigger_fL1DCALV0;   //!
  TBranch        *b_EMCALTrigger_AliVCaloTrigger_fUniqueID;   //!
  TBranch        *b_EMCALTrigger_AliVCaloTrigger_fBits;   //!
  TBranch        *b_EMCALTrigger_AliVCaloTrigger_fName;   //!
  TBranch        *b_EMCALTrigger_AliVCaloTrigger_fTitle;   //!
  TBranch        *b_EMCALTrigger_fNEntries;   //!
  TBranch        *b_EMCALTrigger_fCurrent;   //!
  TBranch        *b_EMCALTrigger_fColumn;   //!
  TBranch        *b_EMCALTrigger_fRow;   //!
  TBranch        *b_EMCALTrigger_fAmplitude;   //!
  TBranch        *b_EMCALTrigger_fTime;   //!
  TBranch        *b_EMCALTrigger_fNL0Times;   //!
  TBranch        *b_EMCALTrigger_fL1TimeSum;   //!
  TBranch        *b_EMCALTrigger_fTriggerBits;   //!
  TBranch        *b_EMCALTrigger_fL1Threshold;   //!
  TBranch        *b_EMCALTrigger_fL1V0;   //!
  TBranch        *b_EMCALTrigger_fL1FrameMask;   //!
  TBranch        *b_EMCALTrigger_fL1DCALThreshold;   //!
  TBranch        *b_EMCALTrigger_fL1SubRegion;   //!
  TBranch        *b_EMCALTrigger_fL1DCALFrameMask;   //!
  TBranch        *b_EMCALTrigger_fMedian;   //!
  TBranch        *b_EMCALTrigger_fTriggerBitWord;   //!
  TBranch        *b_EMCALTrigger_fL1DCALV0;   //!
  TBranch        *b_SPDPileupVertices_;   //!
  TBranch        *b_SPDPileupVertices_fUniqueID;   //!
  TBranch        *b_SPDPileupVertices_fBits;   //!
  TBranch        *b_SPDPileupVertices_fName;   //!
  TBranch        *b_SPDPileupVertices_fTitle;   //!
  TBranch        *b_SPDPileupVertices_fPosition;   //!
  TBranch        *b_SPDPileupVertices_fSigma;   //!
  TBranch        *b_SPDPileupVertices_fNContributors;   //!
  TBranch        *b_SPDPileupVertices_fNIndices;   //!
  TBranch        *b_SPDPileupVertices_fIndices;   //!
  TBranch        *b_SPDPileupVertices_fCovXX;   //!
  TBranch        *b_SPDPileupVertices_fCovXY;   //!
  TBranch        *b_SPDPileupVertices_fCovYY;   //!
  TBranch        *b_SPDPileupVertices_fCovXZ;   //!
  TBranch        *b_SPDPileupVertices_fCovYZ;   //!
  TBranch        *b_SPDPileupVertices_fCovZZ;   //!
  TBranch        *b_SPDPileupVertices_fSNR;   //!
  TBranch        *b_SPDPileupVertices_fChi2;   //!
  TBranch        *b_SPDPileupVertices_fID;   //!
  TBranch        *b_SPDPileupVertices_fBCID;   //!
  TBranch        *b_TrkPileupVertices_;   //!
  TBranch        *b_TrkPileupVertices_fUniqueID;   //!
  TBranch        *b_TrkPileupVertices_fBits;   //!
  TBranch        *b_TrkPileupVertices_fName;   //!
  TBranch        *b_TrkPileupVertices_fTitle;   //!
  TBranch        *b_TrkPileupVertices_fPosition;   //!
  TBranch        *b_TrkPileupVertices_fSigma;   //!
  TBranch        *b_TrkPileupVertices_fNContributors;   //!
  TBranch        *b_TrkPileupVertices_fNIndices;   //!
  TBranch        *b_TrkPileupVertices_fIndices;   //!
  TBranch        *b_TrkPileupVertices_fCovXX;   //!
  TBranch        *b_TrkPileupVertices_fCovXY;   //!
  TBranch        *b_TrkPileupVertices_fCovYY;   //!
  TBranch        *b_TrkPileupVertices_fCovXZ;   //!
  TBranch        *b_TrkPileupVertices_fCovYZ;   //!
  TBranch        *b_TrkPileupVertices_fCovZZ;   //!
  TBranch        *b_TrkPileupVertices_fSNR;   //!
  TBranch        *b_TrkPileupVertices_fChi2;   //!
  TBranch        *b_TrkPileupVertices_fID;   //!
  TBranch        *b_TrkPileupVertices_fBCID;   //!
  TBranch        *b_Tracks_;   //!
  TBranch        *b_Tracks_fUniqueID;   //!
  TBranch        *b_Tracks_fBits;   //!
  TBranch        *b_Tracks_fX;   //!
  TBranch        *b_Tracks_fAlpha;   //!
  TBranch        *b_Tracks_fP;   //!
  TBranch        *b_Tracks_fC;   //!
  TBranch        *b_Tracks_fTPCFitMap_fUniqueID;   //!
  TBranch        *b_Tracks_fTPCFitMap_fBits;   //!
  TBranch        *b_Tracks_fTPCFitMap_fNbits;   //!
  TBranch        *b_Tracks_fTPCFitMap_fNbytes;   //!
  TBranch        *b_Tracks_fTPCFitMap_fAllBits;   //!
  TBranch        *b_Tracks_fTPCClusterMap_fUniqueID;   //!
  TBranch        *b_Tracks_fTPCClusterMap_fBits;   //!
  TBranch        *b_Tracks_fTPCClusterMap_fNbits;   //!
  TBranch        *b_Tracks_fTPCClusterMap_fNbytes;   //!
  TBranch        *b_Tracks_fTPCClusterMap_fAllBits;   //!
  TBranch        *b_Tracks_fTPCSharedMap_fUniqueID;   //!
  TBranch        *b_Tracks_fTPCSharedMap_fBits;   //!
  TBranch        *b_Tracks_fTPCSharedMap_fNbits;   //!
  TBranch        *b_Tracks_fTPCSharedMap_fNbytes;   //!
  TBranch        *b_Tracks_fTPCSharedMap_fAllBits;   //!
  TBranch        *b_Tracks_fFrTrackID;   //!
  TBranch        *b_Tracks_fFlags;   //!
  TBranch        *b_Tracks_fID;   //!
  TBranch        *b_Tracks_fLabel;   //!
  TBranch        *b_Tracks_fITSLabel;   //!
  TBranch        *b_Tracks_fITSModule;   //!
  TBranch        *b_Tracks_fTPCLabel;   //!
  TBranch        *b_Tracks_fTRDLabel;   //!
  TBranch        *b_Tracks_fTOFindex;   //!
  TBranch        *b_Tracks_fHMPIDqn;   //!
  TBranch        *b_Tracks_fHMPIDcluIdx;   //!
  TBranch        *b_Tracks_fCaloIndex;   //!
  TBranch        *b_Tracks_fKinkIndexes;   //!
  TBranch        *b_Tracks_fV0Indexes;   //!
  TBranch        *b_Tracks_fHMPIDtrkTheta;   //!
  TBranch        *b_Tracks_fHMPIDtrkPhi;   //!
  TBranch        *b_Tracks_fHMPIDsignal;   //!
  TBranch        *b_Tracks_fdTPC;   //!
  TBranch        *b_Tracks_fzTPC;   //!
  TBranch        *b_Tracks_fCddTPC;   //!
  TBranch        *b_Tracks_fCdzTPC;   //!
  TBranch        *b_Tracks_fCzzTPC;   //!
  TBranch        *b_Tracks_fCchi2TPC;   //!
  TBranch        *b_Tracks_fD;   //!
  TBranch        *b_Tracks_fZ;   //!
  TBranch        *b_Tracks_fCdd;   //!
  TBranch        *b_Tracks_fCdz;   //!
  TBranch        *b_Tracks_fCzz;   //!
  TBranch        *b_Tracks_fCchi2;   //!
  TBranch        *b_Tracks_fITSchi2Std;   //!
  TBranch        *b_Tracks_fITSchi2;   //!
  TBranch        *b_Tracks_fTPCchi2;   //!
  TBranch        *b_Tracks_fTPCchi2Iter1;   //!
  TBranch        *b_Tracks_fTRDchi2;   //!
  TBranch        *b_Tracks_fTOFchi2;   //!
  TBranch        *b_Tracks_fHMPIDchi2;   //!
  TBranch        *b_Tracks_fGlobalChi2;   //!
  TBranch        *b_Tracks_fITSsignal;   //!
  TBranch        *b_Tracks_fITSdEdxSamples;   //!
  TBranch        *b_Tracks_fTPCsignal;   //!
  TBranch        *b_Tracks_fTPCsignalS;   //!
  TBranch        *b_Tracks_fTPCPoints;   //!
  TBranch        *b_Tracks_fTRDsignal;   //!
  TBranch        *b_Tracks_fTRDQuality;   //!
  TBranch        *b_Tracks_fTRDBudget;   //!
  TBranch        *b_Tracks_fCaloDx;   //!
  TBranch        *b_Tracks_fCaloDz;   //!
  TBranch        *b_Tracks_fHMPIDtrkX;   //!
  TBranch        *b_Tracks_fHMPIDtrkY;   //!
  TBranch        *b_Tracks_fHMPIDmipX;   //!
  TBranch        *b_Tracks_fHMPIDmipY;   //!
  TBranch        *b_Tracks_fTPCncls;   //!
  TBranch        *b_Tracks_fTPCnclsF;   //!
  TBranch        *b_Tracks_fTPCsignalN;   //!
  TBranch        *b_Tracks_fTPCnclsIter1;   //!
  TBranch        *b_Tracks_fTPCnclsFIter1;   //!
  TBranch        *b_Tracks_fITSncls;   //!
  TBranch        *b_Tracks_fITSClusterMap;   //!
  TBranch        *b_Tracks_fITSSharedMap;   //!
  TBranch        *b_Tracks_fTRDncls;   //!
  TBranch        *b_Tracks_fTRDncls0;   //!
  TBranch        *b_Tracks_fTRDntracklets;   //!
  TBranch        *b_Tracks_fTRDNchamberdEdx;   //!
  TBranch        *b_Tracks_fTRDNclusterdEdx;   //!
  TBranch        *b_Tracks_fTRDnSlices;   //!
  TBranch        *b_Tracks_fTRDslices;   //!
  TBranch        *b_Tracks_fTRDTimBin;   //!
  TBranch        *b_Tracks_fVertexID;   //!
  TBranch        *b_Tracks_fPIDForTracking;   //!
  TBranch        *b_Tracks_fTrackPhiOnEMCal;   //!
  TBranch        *b_Tracks_fTrackEtaOnEMCal;   //!
  TBranch        *b_Tracks_fTrackPtOnEMCal;   //!
  TBranch        *b_Tracks_fNtofClusters;   //!
  TBranch        *b_Tracks_fTOFcluster;   //!
  TBranch        *b_MuonTracks_;   //!
  TBranch        *b_MuonTracks_fUniqueID;   //!
  TBranch        *b_MuonTracks_fBits;   //!
  TBranch        *b_MuonTracks_fInverseBendingMomentum;   //!
  TBranch        *b_MuonTracks_fThetaX;   //!
  TBranch        *b_MuonTracks_fThetaY;   //!
  TBranch        *b_MuonTracks_fZ;   //!
  TBranch        *b_MuonTracks_fBendingCoor;   //!
  TBranch        *b_MuonTracks_fNonBendingCoor;   //!
  TBranch        *b_MuonTracks_fInverseBendingMomentumAtDCA;   //!
  TBranch        *b_MuonTracks_fThetaXAtDCA;   //!
  TBranch        *b_MuonTracks_fThetaYAtDCA;   //!
  TBranch        *b_MuonTracks_fBendingCoorAtDCA;   //!
  TBranch        *b_MuonTracks_fNonBendingCoorAtDCA;   //!
  TBranch        *b_MuonTracks_fInverseBendingMomentumUncorrected;   //!
  TBranch        *b_MuonTracks_fThetaXUncorrected;   //!
  TBranch        *b_MuonTracks_fThetaYUncorrected;   //!
  TBranch        *b_MuonTracks_fZUncorrected;   //!
  TBranch        *b_MuonTracks_fBendingCoorUncorrected;   //!
  TBranch        *b_MuonTracks_fNonBendingCoorUncorrected;   //!
  TBranch        *b_MuonTracks_fCovariances;   //!
  TBranch        *b_MuonTracks_fRAtAbsorberEnd;   //!
  TBranch        *b_MuonTracks_fChi2;   //!
  TBranch        *b_MuonTracks_fChi2MatchTrigger;   //!
  TBranch        *b_MuonTracks_fLocalTrigger;   //!
  TBranch        *b_MuonTracks_fX1Pattern;   //!
  TBranch        *b_MuonTracks_fY1Pattern;   //!
  TBranch        *b_MuonTracks_fX2Pattern;   //!
  TBranch        *b_MuonTracks_fY2Pattern;   //!
  TBranch        *b_MuonTracks_fX3Pattern;   //!
  TBranch        *b_MuonTracks_fY3Pattern;   //!
  TBranch        *b_MuonTracks_fX4Pattern;   //!
  TBranch        *b_MuonTracks_fY4Pattern;   //!
  TBranch        *b_MuonTracks_fMuonClusterMap;   //!
  TBranch        *b_MuonTracks_fHitsPatternInTrigCh;   //!
  TBranch        *b_MuonTracks_fHitsPatternInTrigChTrk;   //!
  TBranch        *b_MuonTracks_fNHit;   //!
  TBranch        *b_MuonTracks_fLabel;   //!
  TBranch        *b_MuonClusters_;   //!
  TBranch        *b_MuonClusters_fUniqueID;   //!
  TBranch        *b_MuonClusters_fBits;   //!
  TBranch        *b_MuonClusters_fXYZ;   //!
  TBranch        *b_MuonClusters_fErrXY;   //!
  TBranch        *b_MuonClusters_fCharge;   //!
  TBranch        *b_MuonClusters_fChi2;   //!
  TBranch        *b_MuonClusters_fNPads;   //!
  TBranch        *b_MuonClusters_fLabel;   //!
  TBranch        *b_MuonPads_;   //!
  TBranch        *b_MuonPads_fUniqueID;   //!
  TBranch        *b_MuonPads_fBits;   //!
  TBranch        *b_MuonPads_fADC;   //!
  TBranch        *b_MuonPads_fCharge;   //!
  TBranch        *b_MuonGlobalTracks_;   //!
  TBranch        *b_MuonGlobalTracks_fUniqueID;   //!
  TBranch        *b_MuonGlobalTracks_fBits;   //!
  TBranch        *b_MuonGlobalTracks_fCharge;   //!
  TBranch        *b_MuonGlobalTracks_fMatchTrigger;   //!
  TBranch        *b_MuonGlobalTracks_fNMFTClusters;   //!
  TBranch        *b_MuonGlobalTracks_fNWrongMFTClustersMC;   //!
  TBranch        *b_MuonGlobalTracks_fMFTClusterPattern;   //!
  TBranch        *b_MuonGlobalTracks_fPx;   //!
  TBranch        *b_MuonGlobalTracks_fPy;   //!
  TBranch        *b_MuonGlobalTracks_fPz;   //!
  TBranch        *b_MuonGlobalTracks_fPt;   //!
  TBranch        *b_MuonGlobalTracks_fP;   //!
  TBranch        *b_MuonGlobalTracks_fEta;   //!
  TBranch        *b_MuonGlobalTracks_fRapidity;   //!
  TBranch        *b_MuonGlobalTracks_fFirstTrackingPointX;   //!
  TBranch        *b_MuonGlobalTracks_fFirstTrackingPointY;   //!
  TBranch        *b_MuonGlobalTracks_fFirstTrackingPointZ;   //!
  TBranch        *b_MuonGlobalTracks_fXAtVertex;   //!
  TBranch        *b_MuonGlobalTracks_fYAtVertex;   //!
  TBranch        *b_MuonGlobalTracks_fRAtAbsorberEnd;   //!
  TBranch        *b_MuonGlobalTracks_fChi2OverNdf;   //!
  TBranch        *b_MuonGlobalTracks_fChi2MatchTrigger;   //!
  TBranch        *b_MuonGlobalTracks_fLabel;   //!
  TBranch        *b_MuonGlobalTracks_fMuonClusterMap;   //!
  TBranch        *b_MuonGlobalTracks_fHitsPatternInTrigCh;   //!
  TBranch        *b_MuonGlobalTracks_fHitsPatternInTrigChTrk;   //!
  TBranch        *b_MuonGlobalTracks_fLoCircuit;   //!
  TBranch        *b_MuonGlobalTracks_fIsConnected;   //!
  TBranch        *b_MuonGlobalTracks_fProdVertexXYZ;   //!
  TBranch        *b_PmdTracks_;   //!
  TBranch        *b_PmdTracks_fUniqueID;   //!
  TBranch        *b_PmdTracks_fBits;   //!
  TBranch        *b_PmdTracks_fX;   //!
  TBranch        *b_PmdTracks_fY;   //!
  TBranch        *b_PmdTracks_fZ;   //!
  TBranch        *b_PmdTracks_fCluADC;   //!
  TBranch        *b_PmdTracks_fCluPID;   //!
  TBranch        *b_PmdTracks_fDet;   //!
  TBranch        *b_PmdTracks_fNcell;   //!
  TBranch        *b_PmdTracks_fSmn;   //!
  TBranch        *b_PmdTracks_fTrackNo;   //!
  TBranch        *b_PmdTracks_fTrackPid;   //!
  TBranch        *b_PmdTracks_fClMatching;   //!
  TBranch        *b_PmdTracks_fSigX;   //!
  TBranch        *b_PmdTracks_fSigY;   //!
  TBranch        *b_AliESDTrdTrigger_TObject_fUniqueID;   //!
  TBranch        *b_AliESDTrdTrigger_TObject_fBits;   //!
  TBranch        *b_AliESDTrdTrigger_fFlags;   //!
  TBranch        *b_TrdTracks_;   //!
  TBranch        *b_TrdTracks_fUniqueID;   //!
  TBranch        *b_TrdTracks_fBits;   //!
  TBranch        *b_TrdTracks_fSector;   //!
  TBranch        *b_TrdTracks_fStack;   //!
  TBranch        *b_TrdTracks_fA;   //!
  TBranch        *b_TrdTracks_fB;   //!
  TBranch        *b_TrdTracks_fC;   //!
  TBranch        *b_TrdTracks_fY;   //!
  TBranch        *b_TrdTracks_fPID;   //!
  TBranch        *b_TrdTracks_fLayerMask;   //!
  TBranch        *b_TrdTracks_fTrackletIndex;   //!
  TBranch        *b_TrdTracks_fFlags;   //!
  TBranch        *b_TrdTracks_fFlagsTiming;   //!
  TBranch        *b_TrdTracks_fReserved;   //!
  TBranch        *b_TrdTracks_fTrackletRefs;   //!
  TBranch        *b_TrdTracks_fTrackMatch;   //!
  TBranch        *b_TrdTracks_fLabel;   //!
  TBranch        *b_TrdTracklets_;   //!
  TBranch        *b_TrdTracklets_fUniqueID;   //!
  TBranch        *b_TrdTracklets_fBits;   //!
  TBranch        *b_TrdTracklets_fHCId;   //!
  TBranch        *b_TrdTracklets_fTrackletWord;   //!
  TBranch        *b_TrdTracklets_fLabel;   //!
  TBranch        *b_V0s_;   //!
  TBranch        *b_V0s_fUniqueID;   //!
  TBranch        *b_V0s_fBits;   //!
  TBranch        *b_V0s_fParamN_fUniqueID;   //!
  TBranch        *b_V0s_fParamN_fBits;   //!
  TBranch        *b_V0s_fParamN_fX;   //!
  TBranch        *b_V0s_fParamN_fAlpha;   //!
  TBranch        *b_V0s_fParamN_fP;   //!
  TBranch        *b_V0s_fParamN_fC;   //!
  TBranch        *b_V0s_fParamP_fUniqueID;   //!
  TBranch        *b_V0s_fParamP_fBits;   //!
  TBranch        *b_V0s_fParamP_fX;   //!
  TBranch        *b_V0s_fParamP_fAlpha;   //!
  TBranch        *b_V0s_fParamP_fP;   //!
  TBranch        *b_V0s_fParamP_fC;   //!
  TBranch        *b_V0s_fEffMass;   //!
  TBranch        *b_V0s_fDcaV0Daughters;   //!
  TBranch        *b_V0s_fChi2V0;   //!
  TBranch        *b_V0s_fPos;   //!
  TBranch        *b_V0s_fPosCov;   //!
  TBranch        *b_V0s_fNmom;   //!
  TBranch        *b_V0s_fPmom;   //!
  TBranch        *b_V0s_fNormDCAPrim;   //!
  TBranch        *b_V0s_fRr;   //!
  TBranch        *b_V0s_fDistSigma;   //!
  TBranch        *b_V0s_fChi2Before;   //!
  TBranch        *b_V0s_fChi2After;   //!
  TBranch        *b_V0s_fCausality;   //!
  TBranch        *b_V0s_fAngle;   //!
  TBranch        *b_V0s_fPointAngleFi;   //!
  TBranch        *b_V0s_fPointAngleTh;   //!
  TBranch        *b_V0s_fPointAngle;   //!
  TBranch        *b_V0s_fPdgCode;   //!
  TBranch        *b_V0s_fNidx;   //!
  TBranch        *b_V0s_fPidx;   //!
  TBranch        *b_V0s_fStatus;   //!
  TBranch        *b_V0s_fNBefore;   //!
  TBranch        *b_V0s_fNAfter;   //!
  TBranch        *b_V0s_fOnFlyStatus;   //!
  TBranch        *b_Cascades_;   //!
  TBranch        *b_Cascades_fUniqueID;   //!
  TBranch        *b_Cascades_fBits;   //!
  TBranch        *b_Cascades_fParamN_fUniqueID;   //!
  TBranch        *b_Cascades_fParamN_fBits;   //!
  TBranch        *b_Cascades_fParamN_fX;   //!
  TBranch        *b_Cascades_fParamN_fAlpha;   //!
  TBranch        *b_Cascades_fParamN_fP;   //!
  TBranch        *b_Cascades_fParamN_fC;   //!
  TBranch        *b_Cascades_fParamP_fUniqueID;   //!
  TBranch        *b_Cascades_fParamP_fBits;   //!
  TBranch        *b_Cascades_fParamP_fX;   //!
  TBranch        *b_Cascades_fParamP_fAlpha;   //!
  TBranch        *b_Cascades_fParamP_fP;   //!
  TBranch        *b_Cascades_fParamP_fC;   //!
  TBranch        *b_Cascades_fEffMass;   //!
  TBranch        *b_Cascades_fDcaV0Daughters;   //!
  TBranch        *b_Cascades_fChi2V0;   //!
  TBranch        *b_Cascades_fPos;   //!
  TBranch        *b_Cascades_fPosCov;   //!
  TBranch        *b_Cascades_fNmom;   //!
  TBranch        *b_Cascades_fPmom;   //!
  TBranch        *b_Cascades_fNormDCAPrim;   //!
  TBranch        *b_Cascades_fRr;   //!
  TBranch        *b_Cascades_fDistSigma;   //!
  TBranch        *b_Cascades_fChi2Before;   //!
  TBranch        *b_Cascades_fChi2After;   //!
  TBranch        *b_Cascades_fCausality;   //!
  TBranch        *b_Cascades_fAngle;   //!
  TBranch        *b_Cascades_fPointAngleFi;   //!
  TBranch        *b_Cascades_fPointAngleTh;   //!
  TBranch        *b_Cascades_fPointAngle;   //!
  TBranch        *b_Cascades_fPdgCode;   //!
  TBranch        *b_Cascades_fNidx;   //!
  TBranch        *b_Cascades_fPidx;   //!
  TBranch        *b_Cascades_fStatus;   //!
  TBranch        *b_Cascades_fNBefore;   //!
  TBranch        *b_Cascades_fNAfter;   //!
  TBranch        *b_Cascades_fOnFlyStatus;   //!
  TBranch        *b_Cascades_fEffMassXi;   //!
  TBranch        *b_Cascades_fChi2Xi;   //!
  TBranch        *b_Cascades_fDcaXiDaughters;   //!
  TBranch        *b_Cascades_fPosXi;   //!
  TBranch        *b_Cascades_fPosCovXi;   //!
  TBranch        *b_Cascades_fBachMom;   //!
  TBranch        *b_Cascades_fBachMomCov;   //!
  TBranch        *b_Cascades_fPdgCodeXi;   //!
  TBranch        *b_Cascades_fBachIdx;   //!
  TBranch        *b_Kinks_;   //!
  TBranch        *b_Kinks_fUniqueID;   //!
  TBranch        *b_Kinks_fBits;   //!
  TBranch        *b_Kinks_fParamDaughter_fUniqueID;   //!
  TBranch        *b_Kinks_fParamDaughter_fBits;   //!
  TBranch        *b_Kinks_fParamDaughter_fX;   //!
  TBranch        *b_Kinks_fParamDaughter_fAlpha;   //!
  TBranch        *b_Kinks_fParamDaughter_fP;   //!
  TBranch        *b_Kinks_fParamDaughter_fC;   //!
  TBranch        *b_Kinks_fParamMother_fUniqueID;   //!
  TBranch        *b_Kinks_fParamMother_fBits;   //!
  TBranch        *b_Kinks_fParamMother_fX;   //!
  TBranch        *b_Kinks_fParamMother_fAlpha;   //!
  TBranch        *b_Kinks_fParamMother_fP;   //!
  TBranch        *b_Kinks_fParamMother_fC;   //!
  TBranch        *b_Kinks_fDist1;   //!
  TBranch        *b_Kinks_fDist2;   //!
  TBranch        *b_Kinks_fPdr;   //!
  TBranch        *b_Kinks_fXr;   //!
  TBranch        *b_Kinks_fPm;   //!
  TBranch        *b_Kinks_fRr;   //!
  TBranch        *b_Kinks_fShapeFactor;   //!
  TBranch        *b_Kinks_fTPCdensity;   //!
  TBranch        *b_Kinks_fAngle;   //!
  TBranch        *b_Kinks_fLab;   //!
  TBranch        *b_Kinks_fIndex;   //!
  TBranch        *b_Kinks_fID;   //!
  TBranch        *b_Kinks_fRow0;   //!
  TBranch        *b_Kinks_fMultiple;   //!
  TBranch        *b_Kinks_fTPCncls;   //!
  TBranch        *b_Kinks_fStatus;   //!
  TBranch        *b_CaloClusters_;   //!
  TBranch        *b_CaloClusters_fUniqueID;   //!
  TBranch        *b_CaloClusters_fBits;   //!
  TBranch        *b_CaloClusters_fNCells;   //!
  TBranch        *b_CaloClusters_fCellsAbsId;   //!
  TBranch        *b_CaloClusters_fCellsAmpFraction;   //!
  TBranch        *b_CaloClusters_fGlobalPos;   //!
  TBranch        *b_CaloClusters_fEnergy;   //!
  TBranch        *b_CaloClusters_fDispersion;   //!
  TBranch        *b_CaloClusters_fChi2;   //!
  TBranch        *b_CaloClusters_fM20;   //!
  TBranch        *b_CaloClusters_fM02;   //!
  TBranch        *b_CaloClusters_fEmcCpvDistance;   //!
  TBranch        *b_CaloClusters_fTrackDx;   //!
  TBranch        *b_CaloClusters_fTrackDz;   //!
  TBranch        *b_CaloClusters_fDistToBadChannel;   //!
  TBranch        *b_CaloClusters_fPID;   //!
  TBranch        *b_CaloClusters_fID;   //!
  TBranch        *b_CaloClusters_fNExMax;   //!
  TBranch        *b_CaloClusters_fClusterType;   //!
  TBranch        *b_CaloClusters_fTOF;   //!
  TBranch        *b_CaloClusters_fCoreEnergy;   //!
  TBranch        *b_CaloClusters_fNLabel;   //!
  TBranch        *b_CaloClusters_fClusterMCEdepFraction;   //!
  TBranch        *b_CaloClusters_fCellsMCEdepFractionMap;   //!
  TBranch        *b_EMCALCells_AliVCaloCells_fUniqueID;   //!
  TBranch        *b_EMCALCells_AliVCaloCells_fBits;   //!
  TBranch        *b_EMCALCells_AliVCaloCells_fName;   //!
  TBranch        *b_EMCALCells_AliVCaloCells_fTitle;   //!
  TBranch        *b_EMCALCells_fNCells;   //!
  TBranch        *b_EMCALCells_fHGLG;   //!
  TBranch        *b_EMCALCells_fCellNumber;   //!
  TBranch        *b_EMCALCells_fAmplitude;   //!
  TBranch        *b_EMCALCells_fTime;   //!
  TBranch        *b_EMCALCells_fEFraction;   //!
  TBranch        *b_EMCALCells_fMCLabel;   //!
  TBranch        *b_EMCALCells_fType;   //!
  TBranch        *b_PHOSCells_AliVCaloCells_fUniqueID;   //!
  TBranch        *b_PHOSCells_AliVCaloCells_fBits;   //!
  TBranch        *b_PHOSCells_AliVCaloCells_fName;   //!
  TBranch        *b_PHOSCells_AliVCaloCells_fTitle;   //!
  TBranch        *b_PHOSCells_fNCells;   //!
  TBranch        *b_PHOSCells_fHGLG;   //!
  TBranch        *b_PHOSCells_fCellNumber;   //!
  TBranch        *b_PHOSCells_fAmplitude;   //!
  TBranch        *b_PHOSCells_fTime;   //!
  TBranch        *b_PHOSCells_fEFraction;   //!
  TBranch        *b_PHOSCells_fMCLabel;   //!
  TBranch        *b_PHOSCells_fType;   //!
  TBranch        *b_AliRawDataErrorLogs_;   //!
  TBranch        *b_AliRawDataErrorLogs_fUniqueID;   //!
  TBranch        *b_AliRawDataErrorLogs_fBits;   //!
  TBranch        *b_AliRawDataErrorLogs_fName;   //!
  TBranch        *b_AliRawDataErrorLogs_fTitle;   //!
  TBranch        *b_AliRawDataErrorLogs_fEventNumber;   //!
  TBranch        *b_AliRawDataErrorLogs_fDdlID;   //!
  TBranch        *b_AliRawDataErrorLogs_fErrorLevel;   //!
  TBranch        *b_AliRawDataErrorLogs_fErrorCode;   //!
  TBranch        *b_AliRawDataErrorLogs_fCount;   //!
  TBranch        *b_AliESDACORDE_TObject_fUniqueID;   //!
  TBranch        *b_AliESDACORDE_TObject_fBits;   //!
  TBranch        *b_AliESDACORDE_fACORDEBitPattern;   //!
  TBranch        *b_AliESDAD_AliVAD_fUniqueID;   //!
  TBranch        *b_AliESDAD_AliVAD_fBits;   //!
  TBranch        *b_AliESDAD_fBBtriggerADA;   //!
  TBranch        *b_AliESDAD_fBGtriggerADA;   //!
  TBranch        *b_AliESDAD_fBBtriggerADC;   //!
  TBranch        *b_AliESDAD_fBGtriggerADC;   //!
  TBranch        *b_AliESDAD_fMultiplicity;   //!
  TBranch        *b_AliESDAD_fAdc;   //!
  TBranch        *b_AliESDAD_fTime;   //!
  TBranch        *b_AliESDAD_fWidth;   //!
  TBranch        *b_AliESDAD_fBBFlag;   //!
  TBranch        *b_AliESDAD_fBGFlag;   //!
  TBranch        *b_AliESDAD_fADATime;   //!
  TBranch        *b_AliESDAD_fADCTime;   //!
  TBranch        *b_AliESDAD_fADATimeError;   //!
  TBranch        *b_AliESDAD_fADCTimeError;   //!
  TBranch        *b_AliESDAD_fADADecision;   //!
  TBranch        *b_AliESDAD_fADCDecision;   //!
  TBranch        *b_AliESDAD_fTriggerChargeA;   //!
  TBranch        *b_AliESDAD_fTriggerChargeC;   //!
  TBranch        *b_AliESDAD_fTriggerBits;   //!
  TBranch        *b_AliESDAD_fIsBB;   //!
  TBranch        *b_AliESDAD_fIsBG;   //!
  TBranch        *b_AliESDAD_fAdcTail;   //!
  TBranch        *b_AliESDAD_fAdcTrigger;   //!
  TBranch        *b_AliTOFHeader_TObject_fUniqueID;   //!
  TBranch        *b_AliTOFHeader_TObject_fBits;   //!
  TBranch        *b_AliTOFHeader_fDefaultEventTimeValue;   //!
  TBranch        *b_AliTOFHeader_fDefaultEventTimeRes;   //!
  TBranch        *b_AliTOFHeader_fNbins;   //!
  TBranch        *b_AliTOFHeader_fTOFtimeResolution;   //!
  TBranch        *b_AliTOFHeader_fT0spread;   //!
  TBranch        *b_AliTOFHeader_fNumberOfTOFclusters;   //!
  TBranch        *b_AliTOFHeader_fNumberOfTOFtrgPads;   //!
  TBranch        *b_CosmicTracks_;   //!
  TBranch        *b_CosmicTracks_fUniqueID;   //!
  TBranch        *b_CosmicTracks_fBits;   //!
  TBranch        *b_CosmicTracks_fX;   //!
  TBranch        *b_CosmicTracks_fAlpha;   //!
  TBranch        *b_CosmicTracks_fP;   //!
  TBranch        *b_CosmicTracks_fC;   //!
  TBranch        *b_CosmicTracks_fESDtrackIndex;   //!
  TBranch        *b_CosmicTracks_fNCluster;   //!
  TBranch        *b_CosmicTracks_fLeverArm;   //!
  TBranch        *b_CosmicTracks_fChi2PerCluster;   //!
  TBranch        *b_CosmicTracks_fImpactD;   //!
  TBranch        *b_CosmicTracks_fImpactZ;   //!
  TBranch        *b_CosmicTracks_fIsReuse;   //!
  TBranch        *b_CosmicTracks_fFindableRatio;   //!
  TBranch        *b_AliESDTOFCluster_;   //!
  TBranch        *b_AliESDTOFCluster_fUniqueID;   //!
  TBranch        *b_AliESDTOFCluster_fBits;   //!
  TBranch        *b_AliESDTOFCluster_fID;   //!
  TBranch        *b_AliESDTOFCluster_fNTOFhits;   //!
  TBranch        *b_AliESDTOFCluster_fNmatchableTracks;   //!
  TBranch        *b_AliESDTOFCluster_fHitIndex;   //!
  TBranch        *b_AliESDTOFCluster_fMatchIndex;   //!
  TBranch        *b_AliESDTOFHit_;   //!
  TBranch        *b_AliESDTOFHit_fUniqueID;   //!
  TBranch        *b_AliESDTOFHit_fBits;   //!
  TBranch        *b_AliESDTOFHit_fTimeRaw;   //!
  TBranch        *b_AliESDTOFHit_fTime;   //!
  TBranch        *b_AliESDTOFHit_fTOT;   //!
  TBranch        *b_AliESDTOFHit_fTOFLabel;   //!
  TBranch        *b_AliESDTOFHit_fL0L1Latency;   //!
  TBranch        *b_AliESDTOFHit_fDeltaBC;   //!
  TBranch        *b_AliESDTOFHit_fTOFchannel;   //!
  TBranch        *b_AliESDTOFMatch_;   //!
  TBranch        *b_AliESDTOFMatch_fUniqueID;   //!
  TBranch        *b_AliESDTOFMatch_fBits;   //!
  TBranch        *b_AliESDTOFMatch_fDx;   //!
  TBranch        *b_AliESDTOFMatch_fDz;   //!
  TBranch        *b_AliESDTOFMatch_fTrackLength;   //!
  TBranch        *b_AliESDTOFMatch_fIntegratedTimes;   //!
  TBranch        *b_AliESDFIT_TObject_fUniqueID;   //!
  TBranch        *b_AliESDFIT_TObject_fBits;   //!
  TBranch        *b_AliESDFIT_fT0;   //!
  TBranch        *b_AliESDFIT_fFITzVertex;   //!
  TBranch        *b_AliESDFIT_fFITtime;   //!
  TBranch        *b_AliESDFIT_fFITamplitude;   //!
  TBranch        *b_AliESDFIT_fT0best;   //!
  TBranch        *b_HLTGlobalTrigger_TNamed_fUniqueID;   //!
  TBranch        *b_HLTGlobalTrigger_TNamed_fBits;   //!
  TBranch        *b_HLTGlobalTrigger_TNamed_fName;   //!
  TBranch        *b_HLTGlobalTrigger_TNamed_fTitle;   //!
  TBranch        *b_HLTGlobalTrigger_fInputObjectInfo_;   //!
  TBranch        *b_HLTGlobalTrigger_fInputObjectInfo_fUniqueID;   //!
  TBranch        *b_HLTGlobalTrigger_fInputObjectInfo_fBits;   //!
  TBranch        *b_HLTGlobalTrigger_fInputObjectInfo_fName;   //!
  TBranch        *b_HLTGlobalTrigger_fInputObjectInfo_fTitle;   //!
  TBranch        *b_HLTGlobalTrigger_fTriggerItems;   //!
  TBranch        *b_HLTGlobalTrigger_fCounters;   //!
  TBranch        *b_fDetectorStatus;   //!
  TBranch        *b_fDAQDetectorPattern;   //!
  TBranch        *b_fDAQAttributes;   //!
  TBranch        *b_fNTPCClusters;   //!

  ESD_t(const char*);
  virtual ~ESD_t();
  virtual Int_t    GetEntry(Long64_t entry);
  virtual void     Init(TTree *tree);
  virtual Bool_t   Notify();
};

// C wrapper to construct ESD
extern "C" {
  ESD_t * esd_new(const char* path) {
    ESD_t* esd = new ESD_t(path);
    return esd;
  }
  int esd_load_next(ESD_t* esd, const long ievent) {
    return esd->GetEntry(ievent);
  }
  void esd_destroy(ESD_t* esd) {
    // esd->fChain->Delete();
    delete esd;
  }

  int tobjarray_getentriesfast(TObjArray* a) {
    // Beware: Elements of a TObjArray can be NULL!
    return a->GetEntriesFast();
  }

  const char* tobjarray_getname_at(TObjArray* a, int i) {
    // Beware: Elements of a TObjArray can be NULL - for your convinience!
    if (a->At(i) != 0) {
      return a->At(i)->GetName();
    } else {
      return "";
    }
  }
  // Prime the ROOT environment for parallel processing
  void setup_root();
}


#endif
