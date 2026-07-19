<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_config extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_config';
        $this->_pk = 'key';
        parent::__construct();
    }

    public function fetch_all() {
        $rows = DB::fetch_all('SELECT * FROM %t', [$this->_table]);
        $result = [];
        foreach ($rows as $row) {
            $result[$row['key']] = $row['value'];
        }
        return $result;
    }

    public function fetch_by_key($key) {
        return DB::result_first('SELECT value FROM %t WHERE `key`=%s', [$this->_table, $key]);
    }

    public function set($key, $value, $data_type = 'string') {
        DB::query('REPLACE INTO %t (`key`, `value`, `data_type`) VALUES (%s, %s, %s)', [
            $this->_table, $key, $value, $data_type
        ]);
    }
}
