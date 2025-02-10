#include <stdio.h>
#include <stdlib.h>

struct student
{
	char *name;
	unsigned int id;
};

struct course
{
	char *name;
	float average_grade;
	struct student *students;
	unsigned int total_students;
};

struct school
{
	struct course *courses;
	unsigned int total_courses;
};

void
print_school (struct school school)
{
	printf ("\n\nDisplaying %d courses\n\n\n", school.total_courses);
	for (unsigned int i = 0; i < school.total_courses; i++)
		{
			struct course *current_course = &school.courses[i];
			printf ("Course: %s\nAverage grade: %.2f\nTotal students: %d\n",
							current_course->name, current_course->average_grade,
							current_course->total_students);

			for (unsigned int j = 0; j < current_course->total_students; j++)
				{
					struct student *current_student = &current_course->students[j];
					printf ("\tName: %s\n\tID: %d\n", current_student->name,
									current_student->id);
				}
		}
	printf ("\n\n");
}

void
free_school (struct school school)
{
	for (unsigned int i = 0; i < school.total_courses; i++)
		{
			struct course *current_course = &school.courses[i];
			for (unsigned int j = 0; j < current_course->total_students; j++)
				{
					struct student *current_student = &current_course->students[j];
					free (current_student->name);
				}

			free (current_course);
		}

	free (school.courses);
}

int
main (void)
{
	struct school school = { 0 };

	printf ("Welcome to our school system!\n");
	printf ("Enter the amount of courses that this school has: ");
	scanf ("%d", &school.total_courses);

	school.courses = calloc (sizeof (struct course), school.total_courses);
	if (!school.courses)
		{
			fprintf (stderr,
							 "There's been an error allocating memory for the courses.\n");
			return EXIT_FAILURE;
		}

	for (unsigned int i = 0; i < school.total_courses; i++)
		{
			struct course *current_course = &school.courses[i];
			current_course->name = calloc (sizeof (char), 100);

			printf ("Enter the course name: ");
			scanf ("%99s", current_course->name);

			printf ("Enter the average grade: ");
			scanf ("%f", &current_course->average_grade);

			printf ("Enter the amount of students: ");
			scanf ("%d", &current_course->total_students);

			current_course->students
					= calloc (sizeof (struct student), current_course->total_students);
			if (!current_course->students)
				{
					fprintf (stderr,
									 "Couldn't allocate memory for the course students.\n");
					return EXIT_FAILURE;
				}

			for (unsigned int j = 0; j < current_course->total_students; j++)
				{
					struct student *current_student = &current_course->students[j];
					current_student->name = calloc (sizeof (char), 100);

					printf ("Enter the student name: ");
					scanf ("%99s", current_student->name);

					printf ("Enter the student id: ");
					scanf ("%d", &current_student->id);
				}

			printf ("\n");
		}

	print_school (school);
	free_school (school);

	return EXIT_SUCCESS;
}
