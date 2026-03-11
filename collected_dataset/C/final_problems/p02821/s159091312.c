#include<stdio.h>
int partition(int arr[], int low, int high){
    int key;
    key = arr[low];
    while(low<high){
        while(low <high && arr[high]>= key )
            high--;
        if(low<high)
            arr[low++] = arr[high];
        while( low<high && arr[low]<=key )
            low++;
        if(low<high)
            arr[high--] = arr[low];
    }
    arr[low] = key;
    return low;
}
void quick_sort(int arr[], int start, int end){
    int pos;
    if (start<end){
        pos = partition(arr, start, end);
        quick_sort(arr,start,pos-1);
        quick_sort(arr,pos+1,end);
    }
    return;
}

int main()
{
   int n,m;
    int counter = 0;
     int a[100005];
    int ans=0;
    scanf("%d %d",&n,&m);
    for(long long int i =0;i<n;i++)
        scanf("%d",&a[i]);
    //sort();
    quick_sort(a,0,n-1);
    for(int i = n-1;i>=0;i--)
    {
        for( int j = i;j>=0;j--)
        {
            if(i==j||a[i]==a[j])
            {
                ans+=(a[i]+a[j]);
                counter++;
                if(counter>=m)
                    goto end;
            }
            else
            {
                ans+=(a[i]+a[j]);
                counter++;
                if(counter>=m)
                    goto end;
                ans+=(a[j]+a[i]);
                counter++;
                if(counter>=m)
                    goto end;
            }
    
        }
    }
    end:;
    printf("%d",ans);
    return 0;
}

