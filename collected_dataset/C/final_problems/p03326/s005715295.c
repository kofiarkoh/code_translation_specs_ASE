#include<stdio.h>
int main(void){
	long n,m;
	long x[200200],y[200200],z[200200];
	long i,j,h,tmp,flg;
	long max,a[200200],sum;
	scanf("%ld%ld",&n,&m);
	for(i=0;i<n;i++){
		scanf("%ld%ld%ld",&x[i],&y[i],&z[i]);
	}
	max=0;
	for(i=0;i<8;i++){
		for(j=0;j<n;j++){
			if(i&1){
				a[j]=-x[j];
			}else{
				a[j]=x[j];
			}
			if(i&2){
				a[j]-=y[j];
			}else{
				a[j]+=y[j];
			}
			if(i&4){
				a[j]-=z[j];
			}else{
				a[j]+=z[j];
			}
		}
		h=n;flg=1;
		while(h>1||flg){
			if(h>1) h=h/1.3;
			if(h==10||h==9) h=11;
			flg=0;
			for(j=0;j<n-h;j++){
				if(a[j]<a[j+h]){
					tmp=a[j];
					a[j]=a[j+h];
					a[j+h]=tmp;
					flg=1;
				}
			}
		}
		sum=0;
		for(j=0;j<m;j++){
			sum+=a[j];
		}
		if(max<sum) max=sum;
	}
	printf("%ld\n",max);
	return 0;
}
