pub mod weightlifting;
pub mod cardio;

mod diet
{
		pub const NUTRITIONIST: &str = "Norah Nutrition";

		pub fn ask_about_program ()
		{
				println! ("The nutritionist is {}", NUTRITIONIST);
		}
}

use cardio::{ CardioTool, Exercise as CardioExercise, ask_about_program as ask_about_cardio };
use weightlifting::{ Exercise as WeightliftingExercise, ask_about_program as ask_about_weightlifting };
use diet::ask_about_program as ask_about_nutrition;

#[derive (Debug)]
pub struct GymWorkout
{
		cardio: CardioExercise,
		weightlifting: WeightliftingExercise
}

impl GymWorkout
{
		pub fn new () -> Self
		{
				ask_about_nutrition ();
				ask_about_cardio ();
				ask_about_weightlifting ();

				Self
				{
						cardio: CardioExercise::new (String::from ("Monday"), CardioTool::Treadmill, 15),
						weightlifting: WeightliftingExercise::new (String::from ("Tuesday"), 300)
				}
		}
}
