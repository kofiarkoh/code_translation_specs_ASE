#include <stdio.h>
int main(void){
	int N,C,p[100]={},j,i,goukei=0;
	scanf("%d %d",&N,&C);
	for(i=0;i<C;i++){
		scanf("%d",&p[i]);
		goukei+=p[i];
	}
	j=goukei/(N+1);
	if(goukei%(N+1)!=0)printf("%d\n",j+1);
	else printf("%d\n",j);
		
	return 0;
}
