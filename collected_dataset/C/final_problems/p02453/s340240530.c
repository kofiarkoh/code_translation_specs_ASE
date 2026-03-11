#include<stdio.h>
#define N 100000

int check_down(int a[],int x,int idx,int n){

	for(int i=idx;i>=0;i--)
		if(a[i]<=x)return i+1;
		
	return 0;

}

int check_up(int a[],int x,int idx,int n){

	for(int i=idx;i<n;i++)
		if(a[i]>=x)return i;
		
	return n;

}


int lower_bound(int a[],int x,int n){
	int left=0;
	int right=n-1;
	int mid;
	char past='N';
	
	while(left<=right){
		mid=(left+right)/2;
		/*printf("left:%d  mid:%d  right:%d\n",left,mid,right);*/
		
		if(a[mid]==x){
			for(int i=mid;i>=0;i--)
				if(a[i]!=x)return i+1;
			return 0;
		}
		
		else if(a[mid] > x){
			if(past=='R')return check_down(a,x,mid,n);
			right=mid-1;
			past='L';
		}
		else if(a[mid] < x){
			if(past=='L')return check_up(a,x,mid,n);
			left=mid+1;
			past='R';
		}
		
	}
	
	if(past=='L')return 0;
	
	if(past=='R')return n;
	
}



int main(){
	int n,i,q,x;
	int a[N];
	
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
	
		scanf("%d",&x);
		printf("%d\n",lower_bound(a,x,n));
	
	}
	
	return 0;
	
}
