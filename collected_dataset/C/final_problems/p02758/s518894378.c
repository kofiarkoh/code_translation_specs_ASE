#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#define MOD 998244353
void merge_sort (int *array, int *array2,int *work,int *work2,int left, int right) {
  int i, j, k, mid;
  if (left < right) {
    mid = (left + right)/2;
    merge_sort(array, array2,work,work2,left, mid);
    merge_sort(array, array2,work,work2,mid+1, right);
    for (i = mid; i >= left; i--) {
        work[i] = array[i]; 
        work2[i]=array2[i];
        } 
    for (j = mid+1; j <= right; j++) {
        work[right-(j-(mid+1))] = array[j];
        work2[right-(j-(mid+1))] = array2[j];
    }
    i = left; j = right;
    for (k = left; k <= right; k++) {
      if (work[i] < work[j]) { array[k] = work[i]; array2[k]=work2[i];i++;}
      else                   { array[k] = work[j]; array2[k]=work2[j];j--;}
    }
  }
}
int search(int *array, int n, int left ,int right){
    int mid=(left+right)/2;
    if(left < right){
        if(n <= array[mid])search(array, n, left, mid);
        else search(array, n, mid+1, right);
    }else{
        if(n<=array[right]) return right-1;
        else return right;
    }
}
void init(int *tree,int size){
    for (int i = 0; i < 2*size-1; i++)tree[i]=0;    
}
int max(int a,int b){
    if(a<b)return b;
    else return a;
}
void update(int *tree,int size,int a,int x){
    a+=size-1;
    tree[a]=x;
    while(a>0){
        a=(a-1)/2;
        tree[a]=max(tree[2*a+1],tree[2*a+2]);
    }
}
int query(int *tree,int a,int b,int k,int l,int r){
    if(a==b)return a;
    if(b<=l||r<=a)return 0;
    if(a<=l&&r<=b)return tree[k];
    int vl=query(tree,a,b,2*k+1,l,(l+r)/2);
    int vr=query(tree,a,b,2*k+2,(l+r)/2,r);
    return max(vl,vr);
}
int main(){
    int n;
    scanf("%d",&n);
    int *x,*d,*work,*work2;
    x=(int*)malloc(sizeof(int)*n);
    d=(int*)malloc(sizeof(int)*n);
    work=(int*)malloc(sizeof(int)*n);
    work2=(int*)malloc(sizeof(int)*(n+1));
    for (int i = 0; i < n; i++)scanf("%d %d",&x[i],&d[i]);
    merge_sort(x,d,work,work2,0,n-1);

    int size=pow(2,(int)ceil(log2(n)));
    int *tree;
    tree=(int*)malloc(sizeof(int)*(2*size-1));
    init(tree,size);
    for (int i = n-1; i >= 0; i--)
    {
        update(tree,size,i,i);
        update(tree, size, i ,query(tree,i,search(x,x[i]+d[i],i,n-1),0,0,size-1));
    }
    work2[n]=1;
    for (int i = n-1; i >= 0; i--)
    {
        work2[i]=(work2[i+1]+work2[tree[i+size-1]+1])%MOD;
    }
    
    printf("%d\n",work2[0]);
    
    return 0;
 }
