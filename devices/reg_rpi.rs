// Already initialized by Novusk

fn rpi_init() {   }

fn rpi_end() {    }

module_init!(fn_dev_init, rpi_init);
module_end!(fn_dev_end, rpi_end);
