#include <stdio.h>

int main(void)
{
	int N,E,T;
	scanf("%d%d%d",&N,&E,&T);
	long long int i,j,k=0,x[N+1],num=0,flag=T;

	for(i=0;i<N;i++){
		scanf("%lld",&x[i]);
	}
	x[N]=-1000;
	num=x[0];

	for(i=0;i<N;i++){
		for(j=1;i+j<N;j++){
			if((x[i+j])-x[i]<=T){
				k+=x[i+j]-x[i];
			}else{
				break;
			}
		}
		if(k!=0){
			num+=3*k;
			k=0;
			flag=0;
			i=i+j-2;
		}else{
			if(i+1!=N){
				num+=x[i+1]-x[i]+flag;
			}else{
				num+=E-x[i]+flag;
			}
			flag=T;
		}
	}

	printf("%lld\n",num );

	return 0;
}