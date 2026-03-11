/*考えられるパターン
1.両方とも正でx<y
y-x
2.両方とも正でx>y
符号チェンジ→絶対値を一致→符号チェンジ
3.x=0でyが正
y-x 1.と同じ
4.xが正でyが0
符号チェンジ→x回
5.両方とも負でx<y
y-x
6.両方とも負てx>y
符号チェンジ→絶対値を一致→符号チェンジ
7.x=0でyが負
絶対値を一致→符号チェンジ
8.xが負でyが0
y回
9.xyが正負違う
絶対値の差+符号チェンジ1回

同一の処理もあるが、ミスってたら直しがめんどうなので個別に処理する
処理時間も気にしない、省略せずに実装
*/
#include <stdio.h>
#include <stdlib.h>
int main(void){
    long long int x,y,ans;
    scanf("%lld %lld",&x,&y);
    ans=0;
    if(x>0 && y>0 && x<y){ //パターン1
        ans=y-x;
    }else if(x>0 && y>0 && x>y){ //パターン2
        ans++;
        ans+=(x-y);
        ans++;
    }else if(x==0 && y>0){ //パターン3
        ans=y-x;
    }else if(x>0 && y==0){ //パターン4
        ans++;
        ans+=x;
    }else if(x<0 && y<0 && x<y){ //パターン5
        ans=y-x;
    }else if(x<0 && y<0 && x>y){ //パターン6
        ans++;
        ans+=(x-y);
        ans++;
    }else if(x==0 && y<0){ //パターン7
        ans+=(0-y);
        ans++;
    }else if(x<0 && y==0){ //パターン8
        ans-=x;
    }else if((x>0 && y<0) || (x<0 && y>0) ){ //パターン9
        ans++;
        ans+=llabs((llabs(x)-llabs(y)));
    }
    
    printf("%llu\n",ans);
    return 0;
}