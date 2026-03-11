#include<stdio.h>

void merge(int *,int ,int ,int);
void mergeSort(int *,int,int);

int count=0;

int main(){

  int n,i;
  int S[500000];

  scanf("%d",&n);

  for(i=0;i<n;i++){
    scanf("%d",&S[i]);
  }

  mergeSort(S,0,n);
  
  for(i=0;i<n;i++){
    printf("%d",S[i]);
    if(i==n-1)printf("\n");
    else printf(" ");
  }
  
  printf("%d\n",count);
  
  return 0;
}

void mergeSort(int *s,int l,int r){

  int m;

  if(l+1<r){
    m=(l+r)/2;
    mergeSort(s,l,m);
    mergeSort(s,m,r);
    merge(s,l,m,r);
  }
}

void merge(int *s,int l,int m,int r){

  int n1,n2,i,j,k;

  n1=m-l;
  n2=r-m;

  int L[n1+1],R[n2+1];

  for(i=0;i<=n1-1;i++) L[i]=s[l+i];
  for(i=0;i<=n2-1;i++) R[i]=s[m+i];

  L[n1]=1000000000;
  R[n2]=1000000000;

  i=0;
  k=0;

  for(j=l;j<=r-1;j++){
    count++;

    if(L[i]<=R[k]){
      s[j]=L[i];
      i=i+1;
    }
    else{
      s[j]=R[k];
      k=k+1;
    }
  }
}