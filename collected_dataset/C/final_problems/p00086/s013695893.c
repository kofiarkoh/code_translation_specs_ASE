#include <stdio.h>
#include <math.h>
int p[100];
void reset(){
	int i;
	for(i=0;i<100;i++)p[i]=0;
}
int main(void){
	int i,j;
	reset();
	while(scanf("%d %d",&i,&j)!=EOF){
		if(i>0 && j>0){p[i-1]++;p[j-1]++;if(i>99 || j>99)return 1;}
		else{
		j=1;
		for(j=2;j<100;j++){if(p[j]%2>0){j=0;break;}}
		if(j && (p[0]+p[1])%2==0)printf("OK\n"); else printf("NG\n");
		reset();
		}
	}
	return 0;
}