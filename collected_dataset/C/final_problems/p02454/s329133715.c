#include<stdio.h>
#define N 100000

int lower_bound(int a[],int x,int n){
	int left=0;
	int right=n-1;
	int mid;
	
	while(left<=right){
	
		mid=(left+right)/2;
		
		if(a[mid]==x){
			for(int i=mid;i>=0;i--)
				if(a[i]!=x)return i+1;
			return 0;
		}
		
		else if(a[mid] > x){
			right=mid-1;
		}
		else if(a[mid] < x){
			left=mid+1;
		}
		
	}
	
	
	if(a[mid] < x){
		for(int i=mid;i<n;i++)
			if(a[i] >= x)return i;
		return n;
		
	}
	else {
		for(int i=mid;i>=0;i--)
			if(a[i] <= x)return i+1;
		return 0;
	}

	
}


int upper_bound(int a[],int x,int n){
	int left=0;
	int right=n-1;
	int mid;
	
	while(left<=right){
	
		mid=(left+right)/2;
		
		if(a[mid]==x){
			for(int i=mid;i<n;i++)
				if(a[i]!=x)return i;
			return n;
		}
		
		else if(a[mid] > x){
			right=mid-1;
		}
		else if(a[mid] < x){
			left=mid+1;
		}
		
	}
	
	
	if(a[mid] < x){
		for(int i=mid;i<n;i++)
			if(a[i] > x)return i;
		return n;
		
	}
	else {
		for(int i=mid;i>=0;i--)
			if(a[i] <= x)return i+1;
		return 0;
	}

	
}



int main(){
	int n,i,q,x;
	int a[N];
	
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
	
		scanf("%d",&x);
		printf("%d %d\n",lower_bound(a,x,n),upper_bound(a,x,n));
	
	}
	
	return 0;
	
}
