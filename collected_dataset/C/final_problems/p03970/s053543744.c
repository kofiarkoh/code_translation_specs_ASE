#include<stdio.h>
#include<stdlib.h>

int main() {
	int loop;
	char st[] = "CODEFESTIVAL2016" ;
	char sg[17];
	int ans = 0;

	scanf("%s", sg);

	for (loop = 0; loop < 17; loop++)
		if (st[loop] != sg[loop])ans++;

	printf("%d\n", ans);
	return 0;
}