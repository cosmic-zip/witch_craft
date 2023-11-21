use crate::iso::iso_structs::CybersecurityFramework;

struct CybersecurityFrameworkBuilder {
    framework: CybersecurityFramework,
}

impl CybersecurityFrameworkBuilder {
    fn new() -> Self {
        Self {
            framework: CybersecurityFramework {
                scope: String::new(),
                context: String::new(),
                governance_structure: String::new(),
                cybersecurity_policy: String::new(),
                management_commitment: String::new(),
                risk_management_process: String::new(),
                risk_acceptance_criteria: String::new(),
                risk_treatment_plans: String::new(),
                cybersecurity_controls: Vec::new(),
                control_categories: Vec::new(),
                continuous_monitoring: String::new(),
                incident_response_plan: String::new(),
                incident_response_team: String::new(),
                incident_response_drills: String::new(),
                awareness_program: String::new(),
                training_practices: String::new(),
                cybersecurity_culture: String::new(),
                information_asset_classification: String::new(),
                information_asset_security_measures: Vec::new(),
                security_posture_audit: String::new(),
                third_party_risk_assessment: String::new(),
                third_party_cybersecurity_requirements: String::new(),
                third_party_audit: String::new(),
                monitoring_and_measurement_program: String::new(),
                cybersecurity_reviews_and_audits: String::new(),
                continuous_improvement_process: String::new(),
                compliance: String::new(),
                certification: String::new(),
                framework_update: String::new(),
            },
        }
    }

    // Builder methods for each field, returning a mutable reference to self for method chaining
    fn set_scope(&mut self, scope: &str) -> &mut Self {
        self.framework.scope = scope.to_string();
        self
    }

    fn set_context(&mut self, context: &str) -> &mut Self {
        self.framework.context = context.to_string();
        self
    }

    fn set_governance_structure(&mut self, governance_structure: &str) -> &mut Self {
        self.framework.governance_structure = governance_structure.to_string();
        self
    }

    fn set_cybersecurity_policy(&mut self, cybersecurity_policy: &str) -> &mut Self {
        self.framework.cybersecurity_policy = cybersecurity_policy.to_string();
        self
    }

    fn set_management_commitment(&mut self, management_commitment: &str) -> &mut Self {
        self.framework.management_commitment = management_commitment.to_string();
        self
    }

    fn set_risk_management_process(&mut self, risk_management_process: &str) -> &mut Self {
        self.framework.risk_management_process = risk_management_process.to_string();
        self
    }

    fn set_risk_acceptance_criteria(&mut self, risk_acceptance_criteria: &str) -> &mut Self {
        self.framework.risk_acceptance_criteria = risk_acceptance_criteria.to_string();
        self
    }

    fn set_risk_treatment_plans(&mut self, risk_treatment_plans: &str) -> &mut Self {
        self.framework.risk_treatment_plans = risk_treatment_plans.to_string();
        self
    }

    fn set_cybersecurity_controls(&mut self, cybersecurity_controls: Vec<String>) -> &mut Self {
        self.framework.cybersecurity_controls = cybersecurity_controls;
        self
    }

    fn set_control_categories(&mut self, control_categories: Vec<String>) -> &mut Self {
        self.framework.control_categories = control_categories;
        self
    }

    fn set_continuous_monitoring(&mut self, continuous_monitoring: &str) -> &mut Self {
        self.framework.continuous_monitoring = continuous_monitoring.to_string();
        self
    }

    fn set_incident_response_plan(&mut self, incident_response_plan: &str) -> &mut Self {
        self.framework.incident_response_plan = incident_response_plan.to_string();
        self
    }

    fn set_incident_response_team(&mut self, incident_response_team: &str) -> &mut Self {
        self.framework.incident_response_team = incident_response_team.to_string();
        self
    }

    fn set_incident_response_drills(&mut self, incident_response_drills: &str) -> &mut Self {
        self.framework.incident_response_drills = incident_response_drills.to_string();
        self
    }

    fn set_awareness_program(&mut self, awareness_program: &str) -> &mut Self {
        self.framework.awareness_program = awareness_program.to_string();
        self
    }

    fn set_training_practices(&mut self, training_practices: &str) -> &mut Self {
        self.framework.training_practices = training_practices.to_string();
        self
    }

    fn set_cybersecurity_culture(&mut self, cybersecurity_culture: &str) -> &mut Self {
        self.framework.cybersecurity_culture = cybersecurity_culture.to_string();
        self
    }

    fn set_information_asset_classification(
        &mut self,
        information_asset_classification: &str,
    ) -> &mut Self {
        self.framework.information_asset_classification = information_asset_classification.to_string();
        self
    }

    fn set_information_asset_security_measures(
        &mut self,
        information_asset_security_measures: Vec<String>,
    ) -> &mut Self {
        self.framework.information_asset_security_measures = information_asset_security_measures;
        self
    }

    fn set_security_posture_audit(&mut self, security_posture_audit: &str) -> &mut Self {
        self.framework.security_posture_audit = security_posture_audit.to_string();
        self
    }

    fn set_third_party_risk_assessment(
        &mut self,
        third_party_risk_assessment: &str,
    ) -> &mut Self {
        self.framework.third_party_risk_assessment = third_party_risk_assessment.to_string();
        self
    }

    fn set_third_party_cybersecurity_requirements(
        &mut self,
        third_party_cybersecurity_requirements: &str,
    ) -> &mut Self {
        self.framework.third_party_cybersecurity_requirements =
            third_party_cybersecurity_requirements.to_string();
        self
    }

    fn set_third_party_audit(&mut self, third_party_audit: &str) -> &mut Self {
        self.framework.third_party_audit = third_party_audit.to_string();
        self
    }

    fn set_monitoring_and_measurement_program(
        &mut self,
        monitoring_and_measurement_program: &str,
    ) -> &mut Self {
        self.framework.monitoring_and_measurement_program =
            monitoring_and_measurement_program.to_string();
        self
    }

    fn set_cybersecurity_reviews_and_audits(
        &mut self,
        cybersecurity_reviews_and_audits: &str,
    ) -> &mut Self {
        self.framework.cybersecurity_reviews_and_audits =
            cybersecurity_reviews_and_audits.to_string();
        self
    }

    fn set_continuous_improvement_process(
        &mut self,
        continuous_improvement_process: &str,
    ) -> &mut Self {
        self.framework.continuous_improvement_process = continuous_improvement_process.to_string();
        self
    }

    fn set_compliance(&mut self, compliance: &str) -> &mut Self {
        self.framework.compliance = compliance.to_string();
        self
    }

    fn set_certification(&mut self, certification: &str) -> &mut Self {
        self.framework.certification = certification.to_string();
        self
    }

    fn set_framework_update(&mut self, framework_update: &str) -> &mut Self {
        self.framework.framework_update = framework_update.to_string();
        self
    }

    // Build method to finalize the construction and return the built object
    fn build(self) -> CybersecurityFramework {
        // Optionally perform any additional checks or validations before returning
        self.framework
    }
}

pub fn builder() -> CybersecurityFrameworkBuilder {
    let framework = CybersecurityFrameworkBuilder::new()
        .set_scope("Organization-wide")
        .set_context("Business Environment")
        .set_governance_structure("Some structure")
        .set_cybersecurity_policy("Some policy")
        .set_management_commitment("Some commitment")
        .set_risk_management_process("Some process")
        .set_risk_acceptance_criteria("Some criteria")
        .set_risk_treatment_plans("Some plans")
        .set_cybersecurity_controls(vec!["Control1".to_string(), "Control2".to_string()])
        .set_control_categories(vec!["Category1".to_string(), "Category2".to_string()])
        .set_continuous_monitoring("Some monitoring")
        .set_incident_response_plan("Some plan")
        .set_incident_response_team("Some team")
        .set_incident_response_drills("Some drills")
        .set_awareness_program("Some program")
        .set_training_practices("Some practices")
        .set_cybersecurity_culture("Some culture")
        .set_information_asset_classification("Some classification")
        .set_information_asset_security_measures(vec![
            "Measure1".to_string(),
            "Measure2".to_string(),
        ])
        .set_security_posture_audit("Some audit")
        .set_third_party_risk_assessment("Some assessment")
        .set_third_party_cybersecurity_requirements("Some requirements")
        .set_third_party_audit("Some audit")
        .set_monitoring_and_measurement_program("Some program")
        .set_cybersecurity_reviews_and_audits("Some reviews and audits")
        .set_continuous_improvement_process("Some process")
        .set_compliance("Some compliance")
        .set_certification("Some certification")
        .set_framework_update("Some update")
        .build();

    return framework;
}