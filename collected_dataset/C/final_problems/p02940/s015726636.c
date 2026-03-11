#include <stdio.h>
#include <string.h>

#define MAX 100000
#define MOD 998244353

int main(void){
  long i;
  long long ans=1;
  int fin=-1,same_fin=0;

  long n;
  scanf("%ld",&n);

  char str[3*MAX+1];
  scanf(" %s",str);

  long color_num[3]={};
  for(i=0;i<strlen(str);i++){
    if(str[i]=='R'){
      if(color_num[0]>=color_num[1]&&color_num[0]>=color_num[2]){
        ans=(ans*(n-color_num[0]))%MOD;
      }else if(color_num[0]<color_num[1]&&color_num[0]<color_num[2]){
        if(fin==0){
          same_fin++;
          ans=(ans*same_fin)%MOD;
        }else{
          fin=0;
          same_fin=1;
        }
      }else{
        if(color_num[1]>color_num[2]){
          ans=(ans*(color_num[1]-color_num[0]))%MOD;
          if(color_num[0]==color_num[2]){
            fin=-1; same_fin=0;
          }
        }else{
          ans=(ans*(color_num[2]-color_num[0]))%MOD;
          if(color_num[0]==color_num[1]){
            fin=-1; same_fin=0;
          }
        }
      }
      color_num[0]++;
    }else if(str[i]=='G'){
      if(color_num[1]>=color_num[0]&&color_num[1]>=color_num[2]){
        ans=(ans*(n-color_num[1]))%MOD;
      }else if(color_num[1]<color_num[0]&&color_num[1]<color_num[2]){
        if(fin==1){
          same_fin++;
          ans=(ans*same_fin)%MOD;
        }else{
          fin=1;
          same_fin=1;
        }
      }else{
        if(color_num[0]>color_num[2]){
          ans=(ans*(color_num[0]-color_num[1]))%MOD;
          if(color_num[1]==color_num[2]){
            fin=-1; same_fin=0;
          }
        }else{
          ans=(ans*(color_num[2]-color_num[1]))%MOD;
          if(color_num[0]==color_num[1]){
            fin=-1; same_fin=0;
          }
        }
      }
      color_num[1]++;
    }else{
      if(color_num[2]>=color_num[0]&&color_num[2]>=color_num[0]){
        ans=(ans*(n-color_num[2]))%MOD;
      }else if(color_num[2]<color_num[0]&&color_num[2]<color_num[1]){
        if(fin==2){
          same_fin++;
          ans=(ans*same_fin)%MOD;
        }else{
          fin=2;
          same_fin=1;
        }
      }else{
        if(color_num[0]>color_num[1]){
          ans=(ans*(color_num[0]-color_num[2]))%MOD;
          if(color_num[1]==color_num[2]){
            fin=-1; same_fin=0;
          }
        }else{
          ans=(ans*(color_num[1]-color_num[2]))%MOD;
          if(color_num[0]==color_num[2]){
            fin=-1; same_fin=0;
          }
        }
      }
      color_num[2]++;
    }
  }

  printf("%lld\n",ans);

  return 0;
}
