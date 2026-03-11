        #include <stdio.h>
         
        int main()
        {
            int A, B, Q;
            long int s[100002];
            long int t[100002];
            long int x;
            scanf("%d %d %d\n",&A, &B, &Q);
            for(int i = 1;  i <= A; i++){
                scanf("%ld\n", &s[i]);
            }
            for(int i = 1;  i <= B; i++){
                scanf("%ld\n", &t[i]);
            }
            
            s[0] = 0;
            t[0] = 0;
            s[A+1] = 10000000001;
            t[B+1] = 10000000001;
            
            for(int i = 0;  i < Q; i++){
                
                scanf("%ld\n", &x);
                
                long int fi = 0;
                long int en = A + 1;
                long int mid;
                long int tmp;
                
                long int err1 = s[A+1]- s[0];
                long int err1_arr;
                
                while(en - fi > 1){
                    mid = (fi + en) /2;
                    tmp = x - s[mid];
                    if(tmp < 0) tmp *= -1;
                    if(tmp <= err1) {
                        err1 = tmp;
                        err1_arr = mid;
                    }
                    //printf("mid = %ld s = %ld\n",mid, s[mid]);
                    if(x < s[mid]){
                       en = mid;
                    }
                    else{
                        fi = mid;
                    }
                }
                
                
                //printf("err1 = %ld arr = %ld\n",err1, err1_arr);
                
                fi = 0;
                en = B + 1;
                long int err2 = t[B+1] -t[0];
                long int err2_arr;
                //printf("fi = %ld ed = %ld\n",fi, en);
                while(en - fi > 1){
                    mid = (fi + en) /2;
                    //printf("mid = %ld t = %ld\n",mid, t[mid]);
                    tmp = x - t[mid];
                    if(tmp < 0) tmp *= -1;
                    if(tmp <= err2) {
                        err2 = tmp;
                        err2_arr = mid;
                    }
                    if(x < t[mid]){
                       en = mid;
                    }
                    else{
                        fi = mid;
                    }
                    //printf("fi = %ld ed = %ld\n",fi, en);
                }
                //long int err2 = x - t[mid];
                //if(err2 < 0) err2 *= -1;
                //printf("err2 = %ld arr2 = %ld\n",err2, err2_arr);
                
                
                fi = 0;
                en = B + 1;
                long int err3 = s[A+1] - s[0];
                while(en - fi > 1){
                    mid = (fi + en) /2;
                    tmp = s[err1_arr] - t[mid];
                    if(tmp < 0) tmp *= -1;
                    if(tmp < err3) {
                        err3 = tmp;
                    }
                    if(s[err1_arr] < t[mid]){
                       en = mid;
                    }
                    else{
                        fi = mid;
                    }
                }
                
                
                fi = 0;
                en = A + 1;
                long int err4 = t[B+1] - t[0];
                while(en - fi > 1){
                    mid = (fi + en) /2;
                    tmp = t[err2_arr] - s[mid];
                    if(tmp < 0) tmp *= -1;
                    if(tmp < err4) {
                        err4 = tmp;
                    }
                    if(t[err2_arr] < s[mid]){
                       en = mid;
                    }
                    else{
                        fi = mid;
                    }
                }
                
                //printf("B2 = %ld\n",tmp);
                long int sum1 = err1 + err3;
                long int sum2 = err2 + err4;
                if(sum1 < sum2){
                    printf("%ld\n",sum1);
                }
                else{
                    printf("%ld\n",sum2);
                }
            }
         
            return 0;
        }