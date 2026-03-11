#include<stdio.h>

int main(){
  char str[200] = {};
  long n = 0;
  long hofu[100] = {};
  long hofucnt = 0;
  long k = 0;
  long i = 0;
  long kaburi = 0;
  long ryotan = 0;
  long hoge = 0;
  long long count = 0;
  
  scanf(" %s",str);
  scanf("%ld",&k);
  //printf("%s",str);
  for(i=0;i<100;i++){
    if(str[i] != '\0'){
      n++;
    }else{
      break;
    }
  }
  //printf("%d",n);
  if(str[0] == str[n-1]){
    ryotan = 1;
  }
  
  hofu[0] = 1;
  hofucnt = 0;
  str[n] = str[0];
  for(i=1;i<=n;i++){
    if(str[i-1] == str[i]){
      hofu[hofucnt]++;
    }else{
      hofucnt++;
      hofu[hofucnt]++;
    }
  }
  
  for(i=0;i<=hofucnt;i++){
    kaburi += hofu[i]/2;
    //printf("%d.",hofu[i]);
  }
  if((ryotan == 1)&&(hofu[0]==1)&&(hofu[hofucnt] == 2)){
    count = kaburi*(k)-1;
    //printf("#");
  }else{
    count = kaburi*(k);
  }
  printf("%lld",count);
  return 0;
  //a/ss/a/ss/a/ss/a/ss/aa/ss/a/ss/a/ss/a/ss/aa/ss/a/ss/a/ss/a/ss/a
}
    