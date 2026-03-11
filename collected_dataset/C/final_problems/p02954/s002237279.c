#include <stdio.h>
#include <math.h>
#include <string.h>

typedef long l;
// typedef long long ll;

int main(void){
//typeofdefをしたときに同じ変数lとかを使うとバグるっぽい？ 
  l i, j;
  l length;
  l r, L;//RLの境目のRLのインデックス
  l n_r, n_l;
  int f;//flag 1でL発見 2でR発見
  char str[(int)pow(10, 5)];

  scanf("%s", str);
  length=strlen(str);

  // printf("%ld\n", length);

  l num[length];//子供の人数
  memset(num, 0, sizeof(num));

  for(i=0; i<length; ){
    //RになってLになってRになるのを検知
    n_r=0;
    n_l=0;
    f=0;
    for(j=i; ;j++){
      // printf("%ld\n", j)
      if(str[j]=='R'){
        // printf("str[%ld]=R\n", j);
        if(f==0){
          n_r++;
        }else if(f==1){
          //jは次のi
          i=j;
          break;
        }
      }else if(str[j]=='L'){
        // printf("str[%ld]=L\n", j);
        if(f==0){
          r=j-1;
          L=j;
        }
        f=1;
        n_l++;
        if(j==length-1){
          i=j;
          break;
        }
      }
    }
    //子供の数の処理
    if((n_r+n_l)%2==0){
      num[r]=(n_r+n_l)/2;
      num[L]=(n_r+n_l)/2;
    }else{
      if(n_r%2==0){
        //n_r偶数、n_l奇数
        num[r]=(n_r/2)+(n_l/2);
        num[L]=(n_r/2)+(n_l/2)+1;
      }else{
        num[r]=(n_r/2)+(n_l/2)+1;
        num[L]=(n_r/2)+(n_l/2);
      }
    }
    if(j==length-1){
      break;
    }
  }

  for(i=0; i<length; i++){
    if(i!=length-1) printf("%ld ", num[i]);
    else printf("%ld\n", num[i]);
  }

  return 0;
}
