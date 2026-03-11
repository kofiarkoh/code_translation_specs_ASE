#include <stdio.h>
int main(void){
	int m,n,q,r,a,i,j,t[100],s;
	for(;scanf("%d%d",&m,&n),m;printf("%d\n",q)){
		for(s=i=q=0;i<n;i++){
			scanf("%d",&a);
			t[i]=a;
			s+=a;
			if(q<a)q=a;
		}
		r=s>1500000?1500000:s;
		for(;q<r;){
			j=m;
			a=q+r>>1;
			s=0;
			for(i=0;i<n;i++){
				s+=t[i];
				if(s>a){
					if(!--j)break;
					s=t[i];
				}
			}
			if(i==n){	r=a;	}
			else{	q=a+1;	}
		}
	}
	return 0;
}