#include <stdio.h>

int main(void) {
	// your code goes here
	int n;
	scanf("%d",&n);
	int a;
	int i;
	int notBreak = 1;
	for(i=0;i<n;i++)
	{
		scanf("%d",&a);
		if(a==notBreak){
			notBreak++;
		}
	}
	if(notBreak == 1)
		printf("-1\n");
	else
		printf("%d\n",n-notBreak+1);
	return 0;
}
