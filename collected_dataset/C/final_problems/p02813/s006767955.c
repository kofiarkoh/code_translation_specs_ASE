#include<stdio.h>
#include<stdlib.h>

int num(int arr[], int n, int used[], int i);
int fact(int n)
{
    int ans;

    if(n==0) return 1;
    ans=n*fact(n-1);

    return ans;
}

int main(void)
{
    int n,p[8],q[8],useda[8]={0},usedb[8]={0},a,b,ans,i;

    scanf("%d",&n);
    for ( i = 0; i < n; i++)
    {
        scanf("%d",p+i);
    }
    for ( i = 0; i < n; i++)
    {
        scanf("%d",q+i);
    }

    a=num(p,n,useda,0);
    b=num(q,n,usedb,0);

    ans=(a-b>0)? a-b:b-a;
    printf("%d",ans);

    return 0;
}

int num(int arr[], int n, int used[], int i)
{
    if(i==n-1) return 0;
    used[arr[i]]=1;
    int now,nokori=arr[i]-1;
    for (int j = 1; j < arr[i]; j++)
    {
        if(used[j]==1) nokori--;
    }
    now=nokori*fact(n-i-1);
    int partial=num(arr,n,used,i+1);
    return now+partial;
}