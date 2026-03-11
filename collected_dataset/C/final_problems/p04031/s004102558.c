#include<stdio.h>
void swap(int* a,int* b){
  int temp;
  temp = *a;
  *a = *b;
  *b = temp;
}
void sorted_array(int* arr,int n){
  int i,j;
  int min_i;
  for(i=0;i<n-1;i++){
    min_i = i;
    for(j=i+1;j<n;j++){
      if(arr[j]<min_i){
        min_i=j;
      }
    }
    swap(&arr[i],&arr[min_i]);
  }
}
long long int min_a(long long int a,long long int b){
  if(a>b) return b;
  else return a;
}   
int main(){
  int n;
  scanf("%d\n",&n);
  int arr[n];
  int i;
  for(i=0;i<n;i++){
    scanf("%d ",&arr[i]);
  }
  sorted_array(&arr,n);
  int min,max,j;
  min= arr[0],max= arr[n-1];
  long long int min_sum = 4000000;
  long long int k;
  for(i=min;i<=max;i++){
    long long int sum = 0;
    for(j=0;j<n;j++){
      sum = sum + (arr[j]-i)*(arr[j]-i);
    }
    k = sum;
    long long int d = min_a(min_sum,k);
    if(d==k){
      min_sum = k;
    }
  }
  printf("%lld\n",min_sum);
  return 0;
}