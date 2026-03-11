#include<stdio.h>

#define MAX 500000
#define SENTINEL 200000000

int L[MAX/2+2],R[MAX/2+2];
//int cnt=0;  cnt=nlogn

void merge(int A[],int n,int left,int mid,int right){ //分割と統治
    int n1 = mid-left;
    int n2 = right-mid;
    for(int i=0;i<n1;i++) L[i]=A[left+i];
    for(int i=0;i<n2;i++) R[i]=A[mid+i];
    L[n1]=R[n2]=SENTINEL;  //番兵
    int i=0,j=0;
    for(int k=left;k<right;k++){
        //cnt++;
        if(L[i]<=R[j]){
            A[k]=L[i++];
        }else{
            A[k]=R[j++];
        }
    }
}

void mergeSort(int A[],int n,int left,int right){
    if(left+1<right){  //要素数が2つ以上
        int mid = (left+right)/2;
        mergeSort(A,n,left,mid);
        mergeSort(A,n,mid,right);
        merge(A,n,left,mid,right);
    }
}

int main(void){
    int n,c,k,i,cnt=1,ans=1,temp;
    scanf("%d%d%d",&n,&c,&k);
    int T[n];
    
    for(i=0;i<n;i++){
        scanf("%d",&T[i]);
    }
    
    mergeSort(T,n,0,n);
    temp = T[0];
    
    for(i=1;i<n;i++){
        if(cnt<=2){
            if(T[i]-temp>k){
                ans++;
                temp = T[i];
                cnt = 1;
            }else{
                cnt++;
            }
        }else{ //cnt==3
            ans++;
            temp = T[i];
            cnt = 1;
        }
    }
    printf("%d\n",ans);
    return 0;
}
