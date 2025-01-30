#include <stdio.h>

int
main (void)
{
	float salary = 0.f;
	printf ("Please enter the salary (per hours of the employee): ");
	scanf ("%f", &salary);

	float total_hours = 0.f;
	printf ("Please enter the total worked hours (in a month): ");
	scanf ("%f", &total_hours);

	printf ("The total that must be paid to the employee is: %.2f\n",
					salary * total_hours);

	return 0;
}
