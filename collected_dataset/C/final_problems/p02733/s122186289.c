#include <stdio.h>
#include <math.h>

#define rep(i, s, n) for (int i = s; i <= n; i++)

int H, W, K;
char S[10][1001];
int sums[10][1000];

int getNumWChoko(int pre_w,int pre_h,int now_w,int now_h)
{
    int ret;
    ret = sums[now_h][now_w];
    if(pre_w) ret-=sums[now_h][pre_w-1];
    if(pre_h) ret-=sums[pre_h-1][now_w];
    if(pre_w&&pre_h) ret+=sums[pre_h-1][pre_w-1];

    return ret;
}

int main()
{
    scanf("%d %d %d", &H, &W, &K);
    for (int i = 0; i < H; i++)
    {
        scanf("%s", S[i]);
    }

    //set sums
    rep(i, 0, H - 1)
    {
        rep(j, 0, W - 1)
        {
            sums[i][j] = S[i][j] == '0' ? 0 : 1;
        }
    }
    rep(i, 0, H - 1)
    {
        rep(j, 1, W - 1)
        {
            sums[i][j] += sums[i][j - 1];
        }
    }
    rep(i, 1, H - 1)
    {
        rep(j, 0, W - 1)
        {
            sums[i][j] += sums[i - 1][j];
        }
    }
    //test sums
    /*rep(i,0,H-1){
    rep(j,0,W-1){
      printf("%d ",sums[i][j]);
    }
    printf("\n");
  }*/


    int ans = 100000000;

    // 横全パターンに対し
    int counter = 0;
    int counter_max = pow(2, H - 1);

    rep(i, 0, counter_max - 1)
    {
        //横パターン用意
        int cut_pos_h[H + 1];
        int cut_num_h = 0;
        int cut_pos_now=0;
        int tmp=i;        
        while(tmp>0){
            if(tmp&1)
            {
                cut_pos_h[cut_num_h] = cut_pos_now;
                cut_num_h++;
            }
            cut_pos_now++;
            tmp /=2;
        }
        cut_pos_h[cut_num_h] = H-1;
        cut_pos_h[cut_num_h+1] = -1;

        //ex: cut_pos_h = {0,3,H-1,-1}        

        //横パターンに対応する縦チェック
        int cut_pos_w[W + 1];
        int cut_num_w = 0;
        int pre_w = 0;
        int ok=1;
        rep(j, 0, W - 1)
        {
            int pre_h=0, now_h;
            rep(k,0,H-1){
                now_h=cut_pos_h[k];
                if(now_h==-1) break;

                int num = getNumWChoko(pre_w,pre_h,j,now_h);
                if(num>K)
                {
                    pre_w=j;
                    cut_pos_w[cut_num_w]=j;
                    cut_num_w++;
                    num = getNumWChoko(pre_w,pre_h,j,now_h);
                    if(num>K)
                    {
                        ok=0;break;
                    }

                }

                pre_h=now_h+1;
            }
            if(!ok) break;
        }

        //okなら回数記録
        if(ok)
        {
            /*
            //test
            rep(j,0,cut_num_h-1)
            {
                printf("h%d, ",cut_pos_h[j]);
            }
            rep(j,0,cut_num_w-1)
            {
                printf("w%d, ",cut_pos_w[j]);
            }
            printf("\n");*/

            int tmp_ans=cut_num_w+cut_num_h;
            if(tmp_ans<ans) ans=tmp_ans;
        }

    }

    printf("%d",ans);


    return 0;
}