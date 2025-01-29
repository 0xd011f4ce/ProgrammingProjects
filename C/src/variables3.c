#include <stdio.h>

int
main (void)
{
	int grade1 = 0, grade2 = 0, grade3 = 0;
	float average = 0.f;

	printf ("Please enter the three grades like this: grade grade grade. ");
	scanf ("%d %d %d", &grade1, &grade2, &grade3);

	average = (float)(grade1 + grade2 + grade3) / 3;
	printf ("The average of the grades is: %f\n", average);

	return 0;
}
