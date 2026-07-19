<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_evolution extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_evolution';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_by_from($from_id) {
        return DB::fetch_all('SELECT * FROM %t WHERE from_id=%d', [$this->_table, $from_id]);
    }

    public function fetch_by_to($to_id) {
        return DB::fetch_all('SELECT * FROM %t WHERE to_id=%d', [$this->_table, $to_id]);
    }
}
