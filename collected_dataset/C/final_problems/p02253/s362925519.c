#include<stdio.h>

struct data{
	int start;
	int finish;
};

struct data arr[100005];
struct data temp[100005];
void mergesort(struct data *arr, int left, int right)
{
	if(left==right)
	{
		return;
	}
	
	int mid=(left+right)/2;
	mergesort(arr,left,mid);
	mergesort(arr,mid+1,right);
	
	int l=left;
	int tl=mid;
	int r=mid+1;
	int tr=right;
	int idx=left;
	
	while(l<=tl && r<=tr)
	{
		if(arr[l].finish<arr[r].finish)
		{
			temp[idx]=arr[l++];
		}
		else
		{
			temp[idx]=arr[r++];
		}
		idx++;
	}
	
	while(l<=tl)
	{
		temp[idx++]=arr[l++];
	}
	
	while(r<=tr)
	{
		temp[idx++]=arr[r++];
	}
	
	for(int i=left;i<=right;i++)
	{
		arr[i]=temp[i];
	}
}

int main()
{
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)
	{
		scanf("%d %d",&arr[i].start,&arr[i].finish);		
	}
	
	int count=1;
	int idx=0;
	mergesort(arr,0,n-1);
	
	for(int i=1;i<n;i++)
	{
		if(arr[idx].finish<arr[i].start)
		{
			count++;
			idx=i;
		}
	}
	printf("%d\n",count);
	
}
