#include <stdio.h>

int getLB(int a[],int k,int head,int tail){
    if(a[0]>=k)return 0;
    
    if(head>tail)return -1;
    
    int mid=(head+tail)/2;
    if(a[mid]<k)head=mid+1;
    else if(a[mid]>=k){
        if(a[mid-1]<k)return mid;
        tail=mid-1;
    }
    
    return getLB(a,k,head,tail);
}

int main(void){
    int i;
    
    int n;
    scanf("%d\n",&n);
    int a[n];
    for(i=0;i<n;i++)scanf("%d ",&a[i]);
    
    int q;
    scanf("%d\n",&q);
    
    int k,LB;
    while(q--){
        scanf("%d\n",&k);
        LB=getLB(a,k,0,n-1);
        if(LB==-1)LB=n;
        printf("%d\n",LB);
    }

    return 0;
}

