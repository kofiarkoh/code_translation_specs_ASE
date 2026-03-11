#include <stdio.h>
int main(void)
{
	int n,c,i=0,p,su=0,cake;
	scanf("%d %d",&n,&c);
	for(i=0;i<c;i++){
		scanf("%d",&p);
		su+=p;
	}
	cake=su/(n+1);
	if(su%(n+1)!=0){
		cake++;
	}
	printf("%d\n",cake);
	return 0;
}
