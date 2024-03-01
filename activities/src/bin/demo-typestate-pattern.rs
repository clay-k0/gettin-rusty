struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 70 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

fn main() {
    let employee_one = Employee::new("Clay");
    let onboarding_process = employee_one.read_agreement().sign().train(90);
    match onboarding_process {
        Ok(emp) => println!(
            "{}'s Onbarding is Complete. Score: {}/100",
            emp.name, emp.state.score
        ),
        Err(emp) => println!(
            "{}'s Training Failed. Score: {}/100",
            emp.name, emp.state.score
        ),
    }
}
