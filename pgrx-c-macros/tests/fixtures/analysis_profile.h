enum { ANALYSIS_HEADER_ENUM = 7 };
typedef int AnalysisHeaderType;

#define ANALYSIS_BINARY(value) ((value) + 0b1)
#define ANALYSIS_BINDING_ENUM(value) ((value) + ANALYSIS_HEADER_ENUM)
#define ANALYSIS_BINDING_TYPE(value) ((AnalysisHeaderType) (value))

typedef int AnalysisAttributed __attribute__((aligned(16)));
typedef AnalysisAttributed AnalysisAttributedAlias;
#define ANALYSIS_ATTRIBUTED(value) ((AnalysisAttributed) (value))
#define ANALYSIS_ATTRIBUTED_ALIAS(value) ((AnalysisAttributedAlias) (value))
