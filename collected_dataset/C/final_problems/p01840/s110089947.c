#include<stdio.h>
int main(void){
	
	int N,M,T,a[100],sp,s,k,l,hasi,owa,d,e,f,time;
	
	a[0]=0;
	sp=0;
	owa=0;
	f=0;
	
	scanf("%d %d %d\n",&N,&M,&T);
	
	for(s=1;s<=N;s++){
		scanf("%d",&a[s]);
	}
	
	for(k=1;k<=N;k++){
		
		sp=sp+2*M;
		hasi=a[k]-M;
		e=T-a[k];
		
		if(hasi<owa){
			d=owa-hasi;
			sp=sp-d;
		}
		
		if(e<M){
			
			if(f==0){
				sp=sp+T-owa;
				f=1;
			}
			sp=sp-(a[k]+M-owa);
		}
		owa=a[k]+M;
	}
	
	time=T-sp;
	
	printf("%d\n",time);
	
	
	return 0;
}
