#include <stdio.h>
#define SIZ 1400
#define min(a,b) ((a)<(b)?(a):(b))
int A[SIZ][SIZ];
int m[SIZ][SIZ];
int main(){
	int i,j,k,x,y,r,M=0,t=0,z;
	scanf("%d%d",&y,&x);
	for(j=0;j<y;j++)for(i=0;i<x;i++)scanf("%d",A[j]+i);
	for(j=0;j<y;j++)for(r=i=0;i<x;m[j][i]=r,i++)r=A[j][i]==0?r+1:0;
	for(r=i=0;i<x;i++)for(j=0;j<y;j++)for(M=x,k=j;k<y;k++){
		if(M>m[k][i])M=m[k][i];
		int z=k-j+1;
		if(M<z)break;
		if(r<z)r=z;
	}
	printf("%d\n",r*r);
	return 0;
}