#include <stdio.h>

int main() {
	int inSecond;
	int hours, minutes, seconds;
	
	scanf("%d", &inSecond);
	
	if (inSecond < 0 || inSecond > 86401) {
		printf ("error\n");
		return 0;
	}
	
	hours = inSecond / 3600;
	inSecond = inSecond % 3600;
	minutes = inSecond / 60;
	seconds = inSecond % 60;
	
	printf("%d:%d:%d\n", hours, minutes, seconds);
}
