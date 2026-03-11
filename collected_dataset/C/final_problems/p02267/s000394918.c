#include <stdio.h>

int linearSearch(int *,int );
int n,q;

int main(){
int s[10000],t[50000];
int i,j,key;
int count=0;
  
  
  scanf("%d",&n);
  
  for(i=0;i<n;i++){
    scanf("%d",&s[i]);
  }
  
  scanf("%d",&q);
  
  for(i=0;i<q;i++){
    scanf("%d",&t[i]);
  }

for(i=0;i<q;i++){
key = linearSearch(s,t[i]);

if(key == 1){
count++;
}

}
  
  printf("%d\n",count);
  return 0;
}


int linearSearch(int *S,int T){
int i=0;

S[n] = T;
while(S[i] != T){
i++;
}

      if(i == n) return -1;
      else return 1;
}