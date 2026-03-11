#include <stdio.h>

int binarySearch(int a[],int k,int head,int tail){
    int mid=(head+tail)/2;
//printf("a[%d]=%d,k=%d\n",mid,a[mid],k);
    if(tail<head)return 0;
    
    if(a[mid]==k)return 1;
    else if(a[mid]<k)head=mid+1;
    else if(a[mid]>k)tail=mid-1;
    
    return binarySearch(a,k,head,tail);
}

int main(void){
    int i;
    
    int n;
    scanf("%d\n",&n);
    int a[n];
    for(i=0;i<n;i++)scanf("%d ",&a[i]);
    
    int q;
    scanf("%d\n",&q);
    
    int k;
    for(i=0;i<q;i++){
        scanf("%d\n",&k);
        
        printf("%d\n",binarySearch(a,k,0,n-1));
    }
    
    return 0;
}
