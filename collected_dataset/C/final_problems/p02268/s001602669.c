#include <stdio.h>
int binarysearch(int *,int,int);

int main (){

  int n[100000],nc,q[50000],qc,i,j,count=0;
  scanf("%d",&nc);
  for(i=0 ; i<nc ; i++){
    scanf("%d",&n[i]);
  }
  scanf("%d",&qc);
  for(i=0;i<qc;i++){
    scanf("%d",&q[i]);
    if(q[i] == n[binarysearch(n,nc,q[i])] ){
      count++;
    }
  }

  printf("%d\n",count);
  return 0;
}

  int binarysearch(int *n,int right,int key){
    int i,left=0,mid;
    while(left<right){
      mid = (left + right) / 2;
      if(n[mid] == key){
        return mid;
        break;
      }
      else if(key < n[mid]){
        right = mid;
      }
      else{
        left = mid + 1;
      }
    }
    return 0;
  }