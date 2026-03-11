#include <stdio.h>

int binarySearch(int a[],int k,int head,int tail){
    if(tail<head)return -1;
    
    int mid=(head+tail)/2;
    if(a[mid]==k)return mid;
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
    
    int m;
    scanf("%d\n",&m);
    if(m>n){
        printf("0\n");
        return 0;
    }
    int b[m];
    for(i=0;i<m;i++)scanf("%d ",&b[i]);
    
    int j;
    for(i=0,j=0;i<m;i++){
        j=binarySearch(a,b[i],j,n-1);
        if(j==-1||m-i>n-j){
            printf("0\n");
            return 0;
        }
        j++;
    }
    
    printf("1\n");
    
    return 0;
}

