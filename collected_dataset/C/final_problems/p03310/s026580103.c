#include<stdio.h>
#include<stdlib.h>
#define MAX_LONG_LONG 9223372036854775807

long long int sum(long long int *a,int l,int r){
	int i;
	long long int sum=0;
	for(i=l;i<=r;i++){
		sum+=a[i];
	}
	return(sum);	
}

int compare(const void *a,const void *b){
	return *(int*)a - *(int*)b;
}

int main(){
	int N;
	scanf("%d",&N);
	
	long long int a[N];
	long long int b,c,btoc,d,e,dtoe,ans=MAX_LONG_LONG;
	long long int bcde[4];
	int i;
	for(i=0;i<N;i++){
		scanf("%lld",&a[i]);
	}
	
	int center;
	btoc=a[0];
	dtoe=sum(a,1,N-1);
	
	for(center=1;center<N-1;center++){
		b=0;
		btoc+=a[center];
		for(i=0;i<center;i++){
			b+=a[i];
			if(b*2>=btoc) break;
		}
		if(b*2-btoc <= btoc-(b-a[i])*2){
			c=btoc-b;
		} else{
			b-=a[i];
			c=btoc-b;
		}
		
		d=0;
		dtoe-=a[center];
		for(i=center+1;i<N;i++){
			d+=a[i];
			if(d*2>=dtoe) break;
		}
		if(d*2-dtoe <= dtoe-(d-a[i])*2){
			e=dtoe-d;
		} else{
			d-=a[i];
			e=dtoe-d;
		}
		
		bcde[0]=b;
		bcde[1]=c;
		bcde[2]=d;
		bcde[3]=e;
		qsort(bcde,4,sizeof(long long int),compare);
		if(bcde[3]-bcde[0] < ans) ans=bcde[3]-bcde[0];
	}
	
	printf("%lld\n",ans);
	
	return 0;
}