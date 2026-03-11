#include <stdio.h>

int main(void)
{
	int h, w, k;
	char strs[10][1000];
    int chocolates[10][1000];
	int i, j, l, m;
	int pwr, cut[10], count[10], row;
    int ans, ans_tmp;
    int progressed;

	scanf("%d %d %d ", &h, &w, &k);
	for(i = 0; i < h; i++){
    	scanf("%s ", strs[i]);
    }
  
	for(i = 0; i < h; i++){
    	for(j = 0; j < w; j++){
        	chocolates[i][j] = (strs[i][j] == '1');
        }
    }

	pwr = 1 << (h-1);
	ans = 10000000;
	for(i = 0; i < pwr; i++){
        ans_tmp = 0;
    	for(j = 0; j < 9; j++){
        	cut[j] = (i >> j) & 1;
        	ans_tmp += cut[j];
        }            
      
        for(m = 0; m < h; m++){
            count[m] = 0;
        }
        progressed = 0;
      
    	for(j = 0; j < w; j++, progressed = 1){
        	row = 0;
        	for(l = 0; l < h; l++){
        		count[row] += chocolates[l][j];
            	row += cut[l];
            }

            for(l = 0; l < h; l++){
            	if(count[l] > k){
                    if(progressed){
              			for(m = 0; m < 10; m++){
							count[m] = 0;
        				}
                		ans_tmp++;
                        progressed = 0;
                        j--;
                    	break;
                    }
                    else{
                        j = w + 10;
                        progressed = 0;
                        break;
                    }
            	}
            }
        }
      
    	if(j != w+11 && ans_tmp < ans){
    		ans = ans_tmp;
        }
    }

	printf("%d", ans);


	return 0;
}