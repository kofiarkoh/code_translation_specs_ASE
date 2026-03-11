#include <stdio.h>

int schedule[365];
int number_of_days;
int map[3][3][365];//[x][y][]
int debug;
int dfs(int x, int y, int day, int end, int* moisture);
void print_map(int x, int y, int day, int* dryness);
/**
void print_binary(int bin){
    unsigned int bit=1<<15;
    for(int i=0;i<16;i++){
        if(bin&bit){
            printf("1");
        }else{
            printf("0");
        }
        bit>>=1;
    }
    printf("\n");
}
**/

int main(void){
    int buf[16];
    int dryness[16];
    while(scanf("%d%*c",&number_of_days),number_of_days){
        //?????????????????????
        int i,j;
        for(i=0;i<number_of_days;i++){
            scanf("%d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d", buf, buf+1, buf+2, buf+3, buf+4, buf+5, buf+6, buf+7, buf+8, buf+9, buf+10, buf+11, buf+12, buf+13, buf+14, buf+15);
             for(j=15;~j;j--){
                if(buf[j]==1){
                    schedule[i]++;
                }
                if(j)schedule[i]<<=1;
            }           
            /**
            fread(buf, sizeof(char), 32, stdin);
            for(j=15;~j;j--){
                if(buf[j*2]=='1'){
                    schedule[i]++;
                }
                if(j)schedule[i]<<=1;
            }
            **/
        }
        //????????????????????§???????¨?
        int x,y;
        for(i=0;i<number_of_days;i++){
            for(x=0;x<3;x++){
                for(y=0;y<3;y++){
                    if( ((0b110011)<<(x+4*y))&schedule[i]){
                        map[x][y][i]=0;//??????????????????
                    }else{
                        map[x][y][i]=1;//?????????
                    }
                }
            }
        }
        //?????????
        for(i=0;i<16;i++){
            dryness[i]=0;
        }
        if(number_of_days>8){
            for(i=number_of_days-7;i>=0;i--){
                for(x=0;x<3;x++){
                    for(y=0;y<3;y++){
                        if(map[x][y][i]){
                            if(dfs(x,y,i,i+6,dryness)){
                                map[x][y][i]=0;
                            }
                        }
                    }
                }
            }
        }
        
        /**
        for(i=0;i<number_of_days;i++){
            printf("day %d\n",i);
            for(y=0;y<4;y++){
                for(x=0;x<4;x++){
                    printf("%d", schedule[i]&(1<<(4*y+x))? 1:0 );
                }
                printf("\n");
            }
            for(y=0;y<3;y++){
                for(x=0;x<3;x++){
                    printf("%d", map[x][y][i] );
                }
                printf("\n");
            }
            printf("\n");
        }
        **/
        //??±???????????¢?´¢
        for(i=0;i<16;i++){
            dryness[i]=0;
        }
        if(map[1][1][0]==0){
            printf("0\n");
            continue;
        }
        if(~dfs(1, 1, 0, number_of_days-1, dryness)){
            printf("1\n");
        }else{
            printf("0\n");
        }
    }
    return 0;
}
void print_map(int x, int y, int day, int* dryness){
    char map[16];
    int i;
    for(i=0;i<16;i++){
        map[i]='0'+dryness[i];
    }
    map[x+4*y]='X';
    map[x+4*y+1]='X';
    map[x+4*y+4]='X';
    map[x+4*y+5]='X';
    printf("Day %d :\n",day);
    for(y=0;y<4;y++){
        for(x=0;x<4;x++){
            printf("%c",map[x+4*y]);
        }
        printf("\n");
    }
    printf("\n");
}

int dfs(int x, int y, int day, int end, int* dryness){
    /**
    if(day>debug){
        debug=day;
        printf("%d\n",debug);
    }**/
    //??¨???????????????
    int drystats[16];
    int i;
    for(i=0;i<16;i++){
        drystats[i]=dryness[i]+1;
    }
    drystats[x+4*y]=0;
    drystats[x+4*y+1]=0;
    drystats[x+4*y+4]=0;
    drystats[x+4*y+5]=0;
    for(i=0;i<16;i++){
        if(drystats[i]>6) return -1;//??±???
    }
    if(day==end){
        return 0;//??????
    }
    
    //???????§?????????????(?????°)
    for(i=-2;i<3;i++){
        if(i==0){
            if(map[x][y][day+1]){
                if(~dfs(x,y,day+1,end,drystats)){
                    return 0;
                }
            }
        }else{
            if(x+i>=0&&x+i<3){
                if(map[x+i][y][day+1]){
                    if(~dfs(x+i,y,day+1,end,drystats)){
                        return 0;
                    }
                }
            }
            if(y+i>=0&&y+i<3){
                if(map[x][y+i][day+1]){
                    if(~dfs(x,y+i,day+1,end,drystats)){
                        return 0;
                    }
                }
            }
        }
    }
    return -1;
}