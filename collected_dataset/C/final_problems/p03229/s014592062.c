#include<stdio.h>
#define max(xx,yy) (xx>yy) ? (xx) : (yy)
#define abs(zz) (zz>=0) ? (zz) : (-(zz))

void quicksort(int data[],int left,int right);

int main()
{
    int n,m,i,a,num[100010],ans1=0,ans2=0;
	scanf("%d",&n);
	for(i=0;i<n;i++)
	{
		scanf("%d",&(num[i]));
	}
	quicksort(num,0,n-1);
	//pattern1
	m=n/2;
	for(i=0;i<n;i++)
	{
		a=num[i];
		if(i<m-1)ans1+=2*a;
		else if(i==m-1) ans1+=a;
		else if(i==m&&n%2==1) ans1+=a;
		else if(i==m&&n%2==0) ans1-=a;
		else ans1-=2*a;
	}
	if(n%2==1)
	{
		for(i=0;i<n;i++)
		{
			a=num[i];
			if(i<m)ans2+=2*a;
			else if(i==m||i==m+1) ans2-=a;
			else ans2-=2*a;
		}
	}
	printf("%d",max(ans1,ans2));
}

void quicksort(int data[],int left,int right)
{
	int i,j;
	int std,work;
	if(left<right)
	{
		i=left,j=right;
		std=data[(i+j)/2];//基準
		do
		{
			while(data[i] > std)i++;
			while(data[j] < std)j--;
			if(i<=j)
			{
				if(i<j)
				{
					work=data[i];
					data[i]=data[j];
					data[j]=work;
				}
				i++; j--;
			}
		}while(i<=j);
		if(left<j)quicksort(data,left,j);
		if(i<right)quicksort(data,i,right);
	}
}