#include <stdio.h>
#include <string.h>

int main(){
  char s[200005];  scanf("%s", s);
  long onecount = 0, len = strlen(s), kisuone = 0;
  int zeroinfo[200005][3], zerolen = 0;
  for(int i=0; i<len; i++){
    if(s[i] == '1'){
      onecount++;
      if(i%2==0)  kisuone++;
    }else{
      zeroinfo[zerolen][0] = i+1;
      zeroinfo[zerolen][1] = kisuone;
      zeroinfo[zerolen][2] = onecount;
      zerolen++;
    }
  }
  
  long sum = 0;
  if(onecount % 2){
    sum = ((onecount+1)/2) * ((onecount+1)/2+1) - (onecount+1)/2;
  }else{
    sum = (onecount/2) * (onecount/2 + 1);
  }
  //  printf("sum:%ld  one:%ld\n", sum, onecount);
  long ans = 0;
  for(int i=0; i<zerolen; i++){
    ans = ans + zeroinfo[i][1];
    if(zeroinfo[i][0] % 2){
      ans = ans + (onecount - zeroinfo[i][2])/2;
    }else{
      ans = ans + (onecount - zeroinfo[i][2] + 1)/2;
    }
  }
  ans = ans + sum;

  printf("%ld\n", ans);
  
  return 0;
}
