#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int ascending(char *temp);
int descending(char *temp);
int a[21]={}; int L;

int main(void){
  int count; int _a;
  int large_a, small_a;
  int j,i_j; int lock;

  while(1){
    scanf("%d %d",&a[0],&L);
    if(a[0]==0 && L==0) break;

    char work[7]={};
    count=0; lock=0;

    while(lock==0){
      _a=a[count];
      for(int i=L-1;i>=0;i--){
        work[i]='0'+_a%10;
        _a/=10;
      }
      count++;
      large_a=descending(work);
      small_a=ascending(work);
      a[count]=large_a-small_a;
      for(int i=0;i<count;i++){
        if(a[count]==a[i]){
          j=i;
          i_j=count-i;
          lock++;
        }
      }
    }
    printf("%d %d %d\n",j,a[j],i_j);
  }
  return 0;
}

int ascending(char *temp){//昇順
  for(int i=0;i<L;i++){
    for(int j=i+1;j<L;j++){
      if(temp[i]>temp[j]){
        char c = temp[i];
        temp[i] = temp[j];
        temp[j] = c;
      }
    }
  }
  return atoi(temp);
}
int descending(char *temp){//降順
  for(int i=0;i<L;i++){
    for(int j=i+1;j<L;j++){
      if(temp[i]<temp[j]){
        char c = temp[i];
        temp[i] = temp[j];
        temp[j] = c;
      }
    }
  }
  return atoi(temp);
}

