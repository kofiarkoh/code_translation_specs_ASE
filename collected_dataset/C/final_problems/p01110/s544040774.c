#include<stdio.h>
int main(void){
    int wid, hei, t, p;
    while(1){
        scanf("%d%d%d%d",&wid,&hei,&t,&p);
        if(wid==0) break;
        int mat[500][500];//mat[wid][hei]
        for(int i=0;i<500;i++){
            for(int j=0;j<500;j++){
                if(i<wid&&j<hei) mat[i][j]=1;
                else mat[i][j]=0;
            }
        }
        int nw=0, nh=0;
        /*for(int i=0;i<10;i++){
            for(int j=0;j<10;j++){
                printf("%d ",mat[i][j]);
            }puts("");
        }*/
        for(int i=0;i<t;i++){
            int d, c;
            scanf("%d%d",&d,&c);
            if(d==1){
                nw+=c;
                for(int j=0;j<c;j++){
                    for(int k=0;k<hei;k++){
                        mat[nw+j][nh+k] += mat[nw-j-1][nh+k];
                    }
                }
                wid = (c>wid-c)?c:wid-c;
            }else{
                nh+=c;
                for(int j=0;j<c;j++){
                    for(int k=0;k<wid;k++){
                        mat[nw+k][nh+j] += mat[nw+k][nh-j-1];
                    }
                }
                hei = (c>hei-c)?c:hei-c;
            }
        }
        for(int i=0;i<p;i++){
            int x, y;
            scanf("%d%d",&x,&y);
            printf("%d\n",mat[nw+x][nh+y]);
        }
    }
}

