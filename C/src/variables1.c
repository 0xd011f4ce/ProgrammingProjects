#include <stdio.h>

int
main (void)
{
	int current_year = 0;
	int user_age = 0;

	printf ("Please enter the current year: ");
	scanf ("%d", &current_year);
	printf ("Please enter your age: ");
	scanf ("%d", &user_age);

	printf ("You were born in: %d\n", current_year - user_age);

	return 0;
}
