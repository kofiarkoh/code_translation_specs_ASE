#include<stdio.h>
int main(void)
{
	int n,p[100],i,j,tok,jb,pon;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&p[i]);
	}
	for(i=0;i<101;i++){
		jb=0;
		for(j=0;j<n;j++){
			if(p[j]>=i){
				jb++;
			}
		}
		if(jb<i){
			pon=i-1;
			break;
		}
	}
	printf("%d\n",pon);
	return 0;
}