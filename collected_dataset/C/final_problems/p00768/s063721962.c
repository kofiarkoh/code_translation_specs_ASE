#include <stdio.h>
#include <stdlib.h>



int main(){

    int M, T, P, R;
    int i, j;
    int team[2000][5] = {{0,}};
    int team_tmp[2000][5] = {{0,}};
    int pen[50][10] = {{0,}};
    int m, t, p, r;

    int rank[2000] = {0,};
    int tmp1, tmp0;
    int g, L;
    int f = 0;

    while(1){
        scanf("%d%d%d%d", &M, &T, &P, &R);

        if(M==0 && T==0 && P ==0 && R==0){
            break;
        }

        //loop

        // start
        for(i=0; i<R; i++){
            scanf("%d%d%d%d", &m, &t, &p, &j);

            if(j==0){
                team[t][0] = team[t][0] + pen[t][p] + m;
                team[t][1] ++;
                team_tmp[t][0] = team[t][0];
                team_tmp[t][1] = team[t][1];
            }else{
                pen[t][p] += 20;
            }


        }

        /*
        for(t=1; t<=10; t++){
            //printf("0: %d\n1: %d\n\n", team[t][0], team[t][1]);
        }
        */

        for(t=1; t<=T; t++){
            //printf("0: %d\n1: %d\n\n", team[t][0], team[t][1]);
        }

        //printf("---------------\n");

        for(i=1; i<=T; i++){
            rank[i] = i;
        }

        /*
           for(i=1; i<=T; i++){
           for(L=1; L<=T; L++){

           if(team[L][1] <= team[L+1][1]){
           tmp0 = rank[L];
           rank[L] = rank[L+1];
           rank[L+1] = tmp0;

           tmp0 = team[L][0];
           team[L][0] = team[L+1][0];
           team[L+1][0] = tmp0;

           tmp0 = team[L][1];
           team[L][1] = team[L+1][1];
           team[L+1][1] = tmp0;

           if(team[L][1] == team[L+1][1] && team[L][0] >= team[L+1][0]){
           tmp0 = rank[L];
           rank[L] = rank[L+1];
           rank[L+1] = tmp0;

           tmp0 = team[L][0];
           team[L][0] = team[L+1][0];
           team[L+1][0] = tmp0;

           tmp0 = team[L][1];
           team[L][1] = team[L+1][1];
           team[L+1][1] = tmp0;

           if(team[L][1] == team[L+1][1]){
           tmp0 = rank[L];
           rank[L] = rank[L+1];
           rank[L+1] = tmp0;

           tmp0 = team[L][0];
           team[L][0] = team[L+1][0];
           team[L+1][0] = tmp0;

           tmp0 = team[L][1];
           team[L][1] = team[L+1][1];
           team[L+1][1] = tmp0;
           }
           }
           }
           }
           }
         */


        for(i=1; i<T; i++){
            for(L=i+1; L<=T; L++){

                if(team[i][1] <= team[L][1]){
                    tmp0 = rank[i];
                    rank[i] = rank[L];
                    rank[L] = tmp0;

                    tmp0 = team[i][0];
                    team[i][0] = team[L][0];
                    team[L][0] = tmp0;

                    tmp0 = team[i][1];
                    team[i][1] = team[L][1];
                    team[L][1] = tmp0;
                    if(team[i][1] == team[L][1] && team[i][0] >= team[L][0]){
                        tmp0 = rank[i];
                        rank[i] = rank[L];
                        rank[L] = tmp0;

                        tmp0 = team[i][0];
                        team[i][0] = team[L][0];
                        team[L][0] = tmp0;

                        tmp0 = team[i][1];
                        team[i][1] = team[L][1];
                        team[L][1] = tmp0;

                        if(team[i][0] == team[L][0] && team[i][1] == team[L][1]){
                            tmp0 = rank[i];
                            rank[i] = rank[L];
                            rank[L] = tmp0;
                            tmp0 = team[i][0];
                            team[i][0] = team[L][0];
                            team[L][0] = tmp0;

                            tmp0 = team[i][1];
                            team[i][1] = team[L][1];
                            team[L][1] = tmp0;
                        }
                    }
                }
            }
        }

        if(f!=0){
                printf("\n");
        }
        f++;
        for(t=1; t<=T; t++){
            //printf("0: %d\n1: %d\n\n", team[t][0], team[t][1]);
        }

        printf("%d", rank[1]);
        for(i=2; i<=T; i++){
            if(team[i-1][0] == team[i][0] && team[i-1][1] == team[i][1]){
                printf("=");
            }else{
                printf(",");
            }
            printf("%d", rank[i]);

        }

        //printf("========================================\n\n");

        // init
        for(i=0; i<=T; i++){
            team[i][0] = 0;
            team[i][1] = 0;
        }

        for(i=0; i<2000; i++){
            rank[i] = 0;
        }

        for(i=0; i<50; i++){
            for(L=0; L<10; L++){
                pen[i][L] = 0;
            }
        }

    }

    return 0;
}