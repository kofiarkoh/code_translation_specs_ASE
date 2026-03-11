#include <stdio.h>

int  Max(int a[],int n){
	int ans=-1;
	int i;
	for(i=0; i<n; i++){
		if(ans<a[i])ans=a[i];
	}
	return ans;
}


int main(){
	int m,smoney,year,n,ri,te;
	double nenri;
	int i,j,k,l,x,y,z;
	int mone[10];
	int tanri[10];


	scanf("%d",&m);
	for(i=0; i<m; i++){
		scanf("%d %d %d",&smoney,&year,&n);
		for(j=0; j<n; j++){
			scanf("%d %lf %d",&ri,&nenri,&te);
			mone[j]=smoney;
			tanri[j]=0;
			for(k=0; k<year; k++){
				if(ri==0){
					tanri[j]+=mone[j]*nenri;
					mone[j]-=te;
				}
				else{
					mone[j]*=(1+nenri);
					mone[j]-=te;
				}
			}
			//printf("j:%d  %d %d\n",j,mone[j],tanri[j]);
			if(ri==0)mone[j]+=tanri[j];
		}
		printf("%d\n",Max(mone,n));
	}
	return 0;
}